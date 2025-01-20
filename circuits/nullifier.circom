pragma circom 2.0.0;

include "circomlib/circuits/poseidon.circom";

template Nullifier() {
    
    // Private inputs
    signal input privateKey;
    signal input blindingFactor;
    
    // Public inputs
    signal input ceremonyID;
    signal input nullifier;

    component poseidonHash = Poseidon(3);
    poseidonHash.inputs[0] <== privateKey;
    poseidonHash.inputs[1] <== ceremonyID;
    poseidonHash.inputs[2] <== blindingFactor;

    nullifier === poseidonHash.out;
}

component main{public[ceremonyID, nullifier]}  = Nullifier();
