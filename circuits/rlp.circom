pragma circom 2.0.0;
include "circomlib/circuits/comparators.circom";
include "utils.circom";

  

template rlp(len){
    signal output nonce;
    signal output balance;
    signal output storageHash[32];
    signal output codeHash[32];
    signal input rlp[len];

    // prefix of a list with more than 55 bytes data
    rlp[0] === 0xf8;
    // dataLen <== rlp[1];
    // the nonce should be zero
    rlp[2] === 0x80;
    signal balanceLen;
    // 70 =  1(0xf8) +1(dataLen) + 1(balanceLen) + 1(nonce) + 33 + 33 
    balanceLen <== len - 70;

    // balance prefix (rlp[3]) check
    rlp[3] === 128 + balanceLen;


    // nonce
    nonce <== 0;


    // balance
    // balance stats from th fifth index
    // first index rlp[0] is 0xf8(list prefix)
    // second index rlp[1] is the data length
    // third index rlp[2]should be 0x80 since the nonce is zero
    // fourth index rlp[3]is the length of the balnce(128 + balanceLen)
    component balanceSub = SubArray(len, 32, 8);
    balanceSub.in <== rlp;
    balanceSub.start <== 4;
    balanceSub.end <== 4 + balanceLen;


    component balanceInt = BytesToNum(len - 70);
    for (var i=0; i<len - 70; i++ ){
        balanceInt.bytes[i] <== balanceSub.out[i];
        log(balanceSub.out[i]);
    }

    balance <== balanceInt.num;

    component storageHashSub = SubArray(len, 32, 8);
    storageHashSub.in <== rlp;
    //  4 + balanceLen + 1(storage length 160(128+32))
    storageHashSub.start <== 5 + balanceLen ;
    storageHashSub.end <== 37 + balanceLen;

    component CodeHashSub = SubArray(len, 32, 8);
    CodeHashSub.in <== rlp;
    //  4 + balanceLen + 1(storage length 160(128+32))+ 32 +1)
    CodeHashSub.start <== 38 + balanceLen ;
    CodeHashSub.end <== 70 + balanceLen;

    storageHash <== storageHashSub.out;
    codeHash <==  CodeHashSub.out;

    log(balance);


}


// component main = rlp(78);

