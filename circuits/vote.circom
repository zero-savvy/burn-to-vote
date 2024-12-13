pragma circom  2.2.0;

include "circomlib/circuits/comparators.circom";

template vote() {

    signal input vote;

    // burn address check
    // nullifier check
    // vote check
    component isVoteZero = IsEqual();
    isVoteZero.in[0] <== vote;
    isVoteZero.in[1] <== 0;

    component isVoteOne = IsEqual();
    isVoteOne.in[0] <== vote;
    isVoteOne.in[1] <== 1;

    signal isVoteValid;
    isVoteZero.out * isVoteOne.out ==> isVoteValid ;

    isVoteValid === 1;
}

component main = vote();