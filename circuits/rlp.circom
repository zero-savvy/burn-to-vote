pragma circom  2.2.0;

include "circomlib/circuits/comparators.circom";
include "circomlib/circuits/mux1.circom";

template rlp(byteLength) {

    signal input in[byteLength];
    signal output rlp;



    assert(byteLength != 0);

    signal isOne;
    signal isLess;
    component eq1 = IsEqual();
    eq1.in[0] <== byteLength;
    eq1.in[1] <== 1;
    eq1.out ==> isOne;

    component lt1 = LessThan(byteLength);
    lt1.in[0] <== in[0];
    lt1.in[1] <== 127;
    lt1.out ==> isLess;

    signal isOneByte;
    isOneByte <== isOne * isLess;


    component mux = Mux1();
    mux.c[0] <== 0;
    mux.c[1] <== in[0];
    mux.s <== isOneByte;
    log(mux.out);

    rlp <== mux.out;





}

// template Encode_length(){
//     signal input l;
//     signal input offset;
//     signal output encoded_length;
//     if (l < 56) {
//         encoded_length <== l + offset;
//     } else {

//     }
//     // if the length is the than 56
//     // if the length is less than 256**8
//     // if the length is more than 256**8
//     // raise error

// }



component main = rlp(1);


