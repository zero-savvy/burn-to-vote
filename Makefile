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
	cd circuits && snarkjs powersoftau new bn128 12 setup/pot12_0000.ptau -v
	cd circuits && snarkjs powersoftau contribute setup/pot12_0000.ptau setup/pot12_0001.ptau --entropy=1234 --name="first contribution" -v
	cd circuits && snarkjs powersoftau prepare phase2 setup/pot12_0001.ptau setup/pot12_final.ptau -v


vote_circuit:
	mkdir -p circuits/vote_files
	cd circuits && circom vote.circom --r1cs --wasm --sym --c
	mv circuits/vote_cpp circuits/vote_files/vote_cpp
	mv circuits/vote_js circuits/vote_files/vote_js
	mv circuits/vote_files/vote_cpp/main.cpp circuits/vote_files/vote_cpp/main.cpp.tmp
	python3 scripts/spit_output.py < circuits/vote_files/vote_cpp/main.cpp.tmp > circuits/vote_files/vote_cpp/main.cpp
	rm circuits/vote_files/vote_cpp/main.cpp.tmp
	cd circuits/vote_files/vote_cpp && make
	mv circuits/vote_files.r1cs circuits/vote_files/vote_files.r1cs
	mv circuits/vote_files.sym circuits/vote_files/vote_files.sym 

vote_zkey:
	cd circuits && snarkjs groth16 setup vote_files/vote.r1cs setup/pot12_final.ptau vote_0000.zkey
	mv circuits/vote_0000.zkey circuits/vote_files/vote_0000.zkey
	cd circuits && snarkjs zkey contribute vote_files/vote_0000.zkey vote_files/vote_0001.zkey --entropy=1234 --name="second contribution" -v

vote_vkey:
	cd circuits && snarkjs zkey export verificationkey vote_files/vote_0001.zkey vote_files/verification_key.json




burnAddress_circuit:
	mkdir -p circuits/burnAddress_files
	cd circuits && circom burnAddress.circom --r1cs --wasm --sym --c
	mv circuits/burnAddress_cpp circuits/burnAddress_files/burnAddress_cpp
	mv circuits/burnAddress_js circuits/burnAddress_files/burnAddress_js
	mv circuits/burnAddress_files/burnAddress_cpp/main.cpp circuits/burnAddress_files/burnAddress_cpp/main.cpp.tmp
	python3 scripts/spit_output.py < circuits/burnAddress_files/burnAddress_cpp/main.cpp.tmp > circuits/burnAddress_files/burnAddress_cpp/main.cpp
	rm circuits/burnAddress_files/burnAddress_cpp/main.cpp.tmp
	mv circuits/burnAddress.r1cs circuits/burnAddress_files/burnAddress.r1cs
	mv circuits/burnAddress.sym circuits/burnAddress_files/burnAddress.sym 
burnAddress_witness:
	snarkjs wtns calculate circuits/burnAddress_files/burnAddress_js/burnAddress.wasm inputs/burn_address.json circuits/burnAddress_files/witness.wtns
	snarkjs wtns check circuits/burnAddress_files/burnAddress.r1cs circuits/burnAddress_files/witness.wtns
	snarkjs wtns export json circuits/burnAddress_files/witness.wtns circuits/burnAddress_files/witness.json

burnAddress_proof:
	snarkjs groth16 prove circuits/burnAddress_files/burnAddress_0001.zkey circuits/burnAddress_files/witness.wtns circuits/burnAddress_files/proof.json circuits/burnAddress_files/public.json

burnAddress_zkey:
	snarkjs groth16 setup circuits/burnAddress_files/burnAddress.r1cs circuits/setup/pot12_final.ptau circuits/burnAddress_files/burnAddress_0000.zkey
	snarkjs zkey contribute circuits/burnAddress_files/burnAddress_0000.zkey circuits/burnAddress_files/burnAddress_0001.zkey

burnAddress_vkey:
	snarkjs zkey export verificationkey circuits/burnAddress_files/burnAddress_0001.zkey circuits/burnAddress_files/verification_key.json

burnAddress_proof_verify:
	snarkjs groth16 verify circuits/burnAddress_files/verification_key.json circuits/burnAddress_files/public.json circuits/burnAddress_files/proof.json

burnAddress:clean_burnAddress_circuit burnAddress_circuit burnAddress_witness burnAddress_zkey  burnAddress_zkey burnAddress_proof burnAddress_vkey burnAddress_proof_verify

clean_inputs:
	rm -rf inputs/*
clean_burnAddress_circuit:
	rm -rf circuits/burnAddress_files

# contract commands


deploy:
	cd contracts && forge compile
	cd contracts && forge create src/Voting.sol:Voting  --broadcast --private-key {private_key} --json  > output.json
	