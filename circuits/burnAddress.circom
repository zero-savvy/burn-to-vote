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
    signal input vote;

    component isVoteZero = IsEqual();
    isVoteZero.in[0] <== vote;
    isVoteZero.in[1] <== 0;

    component isVoteOne = IsEqual();
    isVoteOne.in[0] <== vote;
    isVoteOne.in[1] <== 1;

    signal isVoteValid;
    isVoteZero.out + isVoteOne.out ==> isVoteValid ;

    isVoteValid === 1;

    component poseidonHash = Poseidon(5);
    poseidonHash.inputs[0] <== secret;
    poseidonHash.inputs[1] <== ceremonyID;
    poseidonHash.inputs[2] <== blinding_factor;
    poseidonHash.inputs[3] <== random_secret;
    poseidonHash.inputs[4] <== vote;

    component secretHash = Poseidon(2);
    secretHash.inputs[0] <== secret;
    secretHash.inputs[1] <== random_secret;
    secret_commitment <== secretHash.out;




    address <== poseidonHash.out;

}


// component main{public[ceremonyID, vote]}  = BurnAddress(); 