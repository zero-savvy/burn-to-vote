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
    log(rlp[0]);
    rlp[0] === 0xf8;
    // dataLen <== rlp[1];
    // the nonce should be zero
    rlp[2] === 0x80;
    signal balanceLen;
    // 70 =  1(0xf8) +1(dataLen)  + 1(nonce) + 1(balanceLen) + 33 + 33 
    balanceLen <== rlpLen - 70;
    log("balanceLen");
    log(balanceLen);
    log("aaaaa");


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


    component balanceInt = PaddedBytesToNum(32);
    balanceInt.realLen <== balanceLen;
    for (var i=0; i<32; i++ ){
        balanceInt.bytes[i] <== balanceSub.out[i];
        // log(balanceSub.out[i]);
    }

    balance <== balanceInt.num;
    log("balance");
    log(balance);
    log("balanceLen");
    log(balanceLen);
    

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

//     log(storageHash[0]);
//    log( storageHash[31]);
//     log(codeHash[0]);
//     log(codeHash[31]);

}


// component main = rlp(82);



// Account RLP bytesssss: [248, 77, 128, 137, 2, 84, 190, 176, 45, 29, 204, 0, 0, 160, 86, 232, 31, 23, 27, 204, 85, 166, 255, 131, 69, 230, 146, 192, 248, 110, 91, 72, 224, 27, 153, 108, 173, 192, 1, 98, 47, 181, 227, 99, 180, 33, 160, 197, 210, 70, 1, 134, 247, 35, 60, 146, 126, 125, 178, 220, 199, 3, 192, 229, 0, 182, 83, 202, 130, 39, 59, 123, 250, 216, 4, 93, 133, 164, 112]
// Account RLP bytessssslen : 79
// [2025-04-09T17:03:22Z INFO  POB_Anonymous_Voting::commands::burn] Burn address circuit: 
// rlppppp: [15, 8, 4, 13, 8, 0, 8, 9, 0, 2, 5, 4, 11, 14, 11, 0, 2, 13, 1, 13, 12, 12, 0, 0, 0, 0, 10, 0, 5, 6, 14, 8, 1, 15, 1, 7, 1, 11, 12, 12, 5, 5, 10, 6, 15, 15, 8, 3, 4, 5, 14, 6, 9, 2, 12, 0, 15, 8, 6, 14, 5, 11, 4, 8, 14, 0, 1, 11, 9, 9, 6, 12, 10, 13, 12, 0, 0, 1, 6, 2, 2, 15, 11, 5, 14, 3, 6, 3, 11, 4, 2, 1, 10, 0, 12, 5, 13, 2, 4, 6, 0, 1, 8, 6, 15, 7, 2, 3, 3, 12, 9, 2, 7, 14, 7, 13, 11, 2, 13, 12, 12, 7, 0, 3, 12, 0, 14, 5, 0, 0, 11, 6, 5, 3, 12, 10, 8, 2, 2, 7, 3, 11, 7, 11, 15, 10, 13, 8, 0, 4, 5, 13, 8, 5, 10, 4, 7, 0, 0, 0, 0, 0, 0, 0]
// rlppppp bytes: [248, 77, 128, 137, 2, 84, 190, 176, 45, 29, 204, 0, 0, 160, 86, 232, 31, 23, 27, 204, 85, 166, 255, 131, 69, 230, 146, 192, 248, 110, 91, 72, 224, 27, 153, 108, 173, 192, 1, 98, 47, 181, 227, 99, 180, 33, 160, 197, 210, 70, 1, 134, 247, 35, 60, 146, 126, 125, 178, 220, 199, 3, 192, 229, 0, 182, 83, 202, 130, 39, 59, 123, 250, 216, 4, 93, 133, 164, 112, 0, 0, 0]
// rlppppp bytes: 82
// node 1: 158
// nonce: 0
// balance: [0, 0, 0, 0, 12, 12, 1, 13, 2, 13, 11, 0, 11, 14, 5, 4, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
// balance: 20
// code hash: [12, 5, 13, 2, 4, 6, 0, 1, 8, 6, 15, 7, 2, 3, 3, 12, 9, 2, 7, 14, 7, 13, 11, 2, 13, 12, 12, 7, 0, 3, 12, 0, 14, 5, 0, 0, 11, 6, 5, 3, 12, 10, 8, 2, 2, 7, 3, 11, 7, 11, 15, 10, 13, 8, 0, 4, 5, 13, 8, 5, 10, 4, 7, 0]
// code hash: [197, 210, 70, 1, 134, 247, 35, 60, 146, 126, 125, 178, 220, 199, 3, 192, 229, 0, 182, 83, 202, 130, 39, 59, 123, 250, 216, 4, 93, 133, 164, 112]
// storage hash: [5, 6, 14, 8, 1, 15, 1, 7, 1, 11, 12, 12, 5, 5, 10, 6, 15, 15, 8, 3, 4, 5, 14, 6, 9, 2, 12, 0, 15, 8, 6, 14, 5, 11, 4, 8, 14, 0, 1, 11, 9, 9, 6, 12, 10, 13, 12, 0, 0, 1, 6, 2, 2, 15, 11, 5, 14, 3, 6, 3, 11, 4, 2, 1]
// storage_hash: [86, 232, 31, 23, 27, 204, 85, 166, 255, 131, 69, 230, 146, 192, 248, 110, 91, 72, 224, 27, 153, 108, 173, 192, 1, 98, 47, 181, 227, 99, 180, 33]
// storagae hash: 9


// balance: 44000000000000000000
// balance: [0, 0, 0, 0, 3, 0, 12, 5, 14, 0, 6, 6, 9, 15, 6, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
// balance: 20



// 
// nonce: 0
// balance: 45000000000000000000
// balance: [0, 0, 0, 0, 9, 4, 6, 12, 9, 4, 1, 13, 8, 0, 7, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
// balance: 20
// code hash: [12, 5, 13, 2, 4, 6, 0, 1, 8, 6, 15, 7, 2, 3, 3, 12, 9, 2, 7, 14, 7, 13, 11, 2, 13, 12, 12, 7, 0, 3, 12, 0, 14, 5, 0, 0, 11, 6, 5, 3, 12, 10, 8, 2, 2, 7, 3, 11, 7, 11, 15, 10, 13, 8, 0, 4, 5, 13, 8, 5, 10, 4, 7, 0]
// code hash: [197, 210, 70, 1, 134, 247, 35, 60, 146, 126, 125, 178, 220, 199, 3, 192, 229, 0, 182, 83, 202, 130, 39, 59, 123, 250, 216, 4, 93, 133, 164, 112]
// storage hash: [5, 6, 14, 8, 1, 15, 1, 7, 1, 11, 12, 12, 5, 5, 10, 6, 15, 15, 8, 3, 4, 5, 14, 6, 9, 2, 12, 0, 15, 8, 6, 14, 5, 11, 4, 8, 14, 0, 1, 11, 9, 9, 6, 12, 10, 13, 12, 0, 0, 1, 6, 2, 2, 15, 11, 5, 14, 3, 6, 3, 11, 4, 2, 1]
// storage_hash: [86, 232, 31, 23, 27, 204, 85, 166, 255, 131, 69, 230, 146, 192, 248, 110, 91, 72, 224, 27, 153, 108, 173, 192, 1, 98, 47, 181, 227, 99, 180, 33]
// storagae hash: 9
// 2, 112, 128, 29, 148, 108, 148, 0, 0,