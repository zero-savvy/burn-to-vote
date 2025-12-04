pragma circom  2.2.0;
include "circomlib/circuits/poseidon.circom";
include "circomlib/circuits/comparators.circom";


template BurnAddress() {

    signal output address;
    signal output secret_commitment;
    signal input secret;
    signal input blinding_factor;
    signal input ceremonyID;
    signal input random_secret;
    signal input votingBlockHash;
    signal input action_value;



    component poseidonHash = Poseidon(6);
    poseidonHash.inputs[0] <== secret;
    poseidonHash.inputs[1] <== ceremonyID;
    poseidonHash.inputs[2] <== blinding_factor;
    poseidonHash.inputs[3] <== random_secret;
    poseidonHash.inputs[4] <== votingBlockHash;
    poseidonHash.inputs[5] <== action_value;

    component secretHash = Poseidon(2);
    secretHash.inputs[0] <== secret;
    secretHash.inputs[1] <== random_secret;
    secret_commitment <== secretHash.out;




    address <== poseidonHash.out;

}


// component main{public[ceremonyID, action_value]}  = BurnAddress(); 