# circuit commands

rapidSnark:
	cd circuits/rapidsnark && git submodule init
	cd circuits/rapidsnark && git submodule update
	cd circuits/rapidsnark && ./build_gmp.sh macos_arm64
	cd circuits/rapidsnark && mkdir build_prover_macos_arm64 && cd build_prover_macos_arm64
	cd circuits/rapidsnark && cmake .. -DTARGET_PLATFORM=macos_arm64 -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=../package_macos_arm64
	cd circuits/rapidsnark && make -j4 && make install


trusted_setup:
	mkdir -p circuits/setup
	cd circuits && snarkjs powersoftau new bn128 24 setup/pot12_0000.ptau -v
	cd circuits && snarkjs powersoftau contribute setup/pot12_0000.ptau setup/pot12_0001.ptau --entropy=1234 --name="first contribution" -v
	cd circuits && snarkjs powersoftau prepare phase2 setup/pot12_0001.ptau setup/pot12_final.ptau -v

burnAddress_circuit:
	mkdir -p circuits/burnAddress
	circom circuits/burnAddress.circom --r1cs --wasm --sym -o circuits/burnAddress -l ./node_modules

nullifier_circuit:
	mkdir -p circuits/nullifier
	circom circuits/nullifier.circom --r1cs --wasm --sym -o circuits/nullifier -l ./node_modules
merkleTree_circuit:
	mkdir -p circuits/merkleTree
	circom circuits/merkleTree.circom --r1cs --wasm --sym -o circuits/merkleTree -l ./node_modules

rlp_circuit:
	mkdir -p circuits/rlp
	circom circuits/rlp.circom --r1cs --wasm --sym -o circuits/rlp -l ./node_modules
	snarkjs wtns calculate circuits/rlp/rlp_js/rlp.wasm circuits/inputs/rlp.json circuits/rlp/witness.wtns

mpt_circuit:
	mkdir -p circuits/mpt
	circom circuits/mpt.circom --r1cs --wasm --sym --verbose -o circuits/mpt -l ./node_modules
vote:
	mkdir -p circuits/vote
	circom circuits/vote.circom --r1cs --wasm --sym --verbose -o circuits/vote -l ./node_modules

circuits:clean_circuits burnAddress_circuit nullifier_circuit


clean_inputs:
	rm -rf circuits/inputs/*
clean_circuits:
	rm -rf circuits/burnAddress
	rm -rf circuits/nullifier


# contract commands


deploy:
	cd contracts && forge compile
	cd contracts && forge create src/Voting.sol:Voting  --broadcast --private-key {private_key} --json  > output.json
	