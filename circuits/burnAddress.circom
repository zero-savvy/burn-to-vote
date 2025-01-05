pragma circom  2.2.0;
include "circomlib/circuits/poseidon.circom";

template BurnAddress() {

    signal input address;
    signal input privateKey;
    signal input ceremonyID;
    signal input blinding_factor;
    signal input personalID;
    signal input vote;

    signal output hash;

    component poseidonHash = Poseidon(5);
    poseidonHash.inputs[0] <== privateKey;
    poseidonHash.inputs[1] <== ceremonyID;
    poseidonHash.inputs[2] <== blinding_factor;
    poseidonHash.inputs[3] <== personalID;
    poseidonHash.inputs[4] <== vote;

    hash <== poseidonHash.out;
}


component main = BurnAddress(); 