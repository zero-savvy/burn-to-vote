#!/bin/bash

# Define bold green color
BOLD_GREEN="\033[1;32m"
RESET="\033[0m"


DATE_CMD=$(command -v gdate || command -v date)


# Map folder names to input numbers
FOLDERS=(
    "1-attest-(Pi-zkRA)"
    "2-recurse-(R1)"
    "3-aggregate-(R2)"
    "4-optimized-(R2+R1)"
    "5-optimized-(2xR2+R1)"
)

start_time=""
end_time=""

# Check for valid input
if [ -z "$1" ] || [ "$1" -lt 1 ] || [ "$1" -gt 5 ]; then
    echo "Usage: $0 [1-5]"
    exit 1
fi

FOLDER="${FOLDERS[$(( $1 - 1 ))]}"
echo -e "${BOLD_GREEN}=======================================${RESET}"
echo -e "${BOLD_GREEN}🗂  Entering folder: $FOLDER${RESET}"
echo -e "${BOLD_GREEN}=======================================${RESET}"
cd "$FOLDER" || { echo "❌ Failed to enter directory $FOLDER"; exit 1; }

echo -e "\n${BOLD_GREEN}🚀 Step 1: Executing Nargo${RESET}"
echo "---------------------------------------"
nargo execute
echo "✅ Nargo execution complete"


echo -e "\n${BOLD_GREEN}🧩 Step 2: Writing Verification Key${RESET}"
echo "---------------------------------------"

if [ "$1" -eq 1 ]; then
    bb write_vk -v -s ultra_honk -b "./target/attest.json" -o ./target --output_format bytes_and_fields --honk_recursion 1 --init_kzg_accumulator
else
    bb write_vk -v -b "./target/recurse.json" -o ./target --honk_recursion 1
fi

echo "✅ Verification key written"


echo -e "\n${BOLD_GREEN}🚧 Step 3: Proving${RESET}"
echo "---------------------------------------"

if [ "$1" -eq 1 ]; then
    echo "🔧 Using: ultra_honk | attest.json"
    bb prove -v -s ultra_honk -b "./target/attest.json" -w "./target/attest.gz" -o ./target --output_format bytes_and_fields --honk_recursion 1 --recursive --init_kzg_accumulator
else
    echo "🔧 Using: default | recurse.json"
    bb prove -v -b "./target/recurse.json" -w "./target/recurse.gz" -o ./target --recursive 2>&1 | while IFS= read -r line; do
        echo "$line"

        # Detect when CRS is initialized
        if [[ "$line" == Initialized*BN254*prover*CRS* ]]; then
            start_time=$($DATE_CMD +%s%3N)
        fi

        # Detect when "computed opening proof" appears
        if [[ "$line" == computed*opening*proof* ]]; then
            end_time=$($DATE_CMD +%s%3N)
            if [[ -n "$start_time" ]]; then
                elapsed_ms=$((end_time - start_time))
                echo -e "${GREEN} ⏰ Proving phase took ${elapsed_ms} ms${NC}"
            fi
        fi
    done
fi

echo "✅ Proof generation complete"


echo -e "\n${BOLD_GREEN}🔍 Step 4: Verifying Proof${RESET}"
echo "---------------------------------------"
start_time=$($DATE_CMD +%s%3N)
if [ "$1" -eq 1 ]; then
    bb verify -s ultra_honk -k ./target/vk -p ./target/proof
else
    bb verify -k ./target/vk -p ./target/proof
fi
end_time=$($DATE_CMD +%s%3N)
elapsed_ms=$((end_time - start_time))
echo -e "${GREEN} ⏰ Verification phase took ${elapsed_ms} ms${NC}"

echo -e "\n${BOLD_GREEN}🎉 All steps completed successfully in: $FOLDER${RESET}"
echo -e "${BOLD_GREEN}=======================================${RESET}"
