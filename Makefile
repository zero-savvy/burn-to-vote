rapidSnark:
	cd circuits/rapidsnark && git submodule init
	cd circuits/rapidsnark && git submodule update
	cd circuits/rapidsnark && ./build_gmp.sh host
	cd circuits/rapidsnark && mkdir -p build_prover
	cd circuits/rapidsnark && cd build_prover && cmake .. -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=../package
	cd circuits/rapidsnark && cd build_prover && make -j4 && make install
