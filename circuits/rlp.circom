pragma circom 2.0.0;
include "circomlib/circuits/comparators.circom";
include "utils.circom";

  

template Rlp(len){
    signal output nonce;
    signal output balance;
    signal output storageHash[32];
    signal output codeHash[32];
    signal input rlp[len];
    signal input rlpLen;

    // prefix of a list with more than 55 bytes data
    rlp[0] === 0xf8;
    // dataLen <== rlp[1];
    // the nonce should be zero
    rlp[2] === 0x80;
    signal balanceLen;
    signal isLongBalance;
    signal isEqualTo128;
    signal diff;
    signal shortBalance;
    signal longBalance;
    signal finalBalance;


    diff <== rlp[3] - 128;


    // 3 difference balance 
    // balance == 0, rlp[3] == 128
    // balance > 128, rlp[3] = 128+balance_len followed by balance amount from rlp[3 + 1] ... rlp [3+balance_len]
    // balance < 128, rlp[3] == balance

    component lessThan = LessThan(32);
    lessThan.in[0] <== 128;
    lessThan.in[1] <== rlp[3];
    isLongBalance <== lessThan.out;

    component equal128 = IsEqual();
    equal128.in[0] <== rlp[3];
    equal128.in[1] <== 128;
    isEqualTo128 <== equal128.out;

    balanceLen <== isLongBalance * (rlp[3] - 128) + (1 - isLongBalance);

    // Extract balance if balance > 128
    component balanceSub = SubArray(len, 32, 8);
    balanceSub.in <== rlp;
    balanceSub.start <== 4;
    balanceSub.end <== 4 + balanceLen;

    component balanceInt = PaddedBytesToNum(32);
    balanceInt.realLen <== balanceLen;
    for (var i=0; i<32; i++ ){
        balanceInt.bytes[i] <== balanceSub.out[i];
    }

    shortBalance <== (1 - isLongBalance) * rlp[3];
    longBalance <== isLongBalance * balanceInt.num;

    finalBalance <== shortBalance + longBalance;

    balance <== (1 - isEqualTo128) * finalBalance;
    

    component storageHashSub = SubArray(len, 32, 8);
    storageHashSub.in <== rlp;
    //  3 + balanceLen + 1(storage length 160(128+32))
    storageHashSub.start <== 4 + balanceLen + isLongBalance;
    storageHashSub.end <== 36 + balanceLen + isLongBalance;

    component CodeHashSub = SubArray(len, 32, 8);
    CodeHashSub.in <== rlp;
    //  3 + balanceLen + 1(storage length 160(128+32))+ 32 +1)
    CodeHashSub.start <== 37 + balanceLen + isLongBalance;
    CodeHashSub.end <== 69 + balanceLen + isLongBalance;

    storageHash <== storageHashSub.out;
    codeHash <==  CodeHashSub.out;

}


// component main = Rlp(82);
