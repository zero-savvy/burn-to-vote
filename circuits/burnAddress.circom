pragma circom  2.2.0;
include "circomlib/circuits/poseidon.circom";
include "circomlib/circuits/comparators.circom";


template BurnAddress() {

    signal input address;
    signal input privateKey;
    signal input blinding_factor;
    signal input ceremonyID;
    signal input personalID;
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
    poseidonHash.inputs[0] <== privateKey;
    poseidonHash.inputs[1] <== ceremonyID;
    poseidonHash.inputs[2] <== blinding_factor;
    poseidonHash.inputs[3] <== personalID;
    poseidonHash.inputs[4] <== vote;


    address === poseidonHash.out;

}


component main  = BurnAddress(); 