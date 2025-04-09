pragma circom 2.0.0;
// include "circuits/circom-ecdsa/circuits/vocdoni-keccak/keccak.circom";
include "./keccak.circom";
include "./utils.circom";
include "./rlp.circom";


template mpt(maxDepth){
    // , keyHexLen, maxValueHexLen
    // var maxLeafRlpHexLen = 4 + (keyHexLen + 2) + 4 + maxValueHexLen;
    // var maxBranchRlpHexLen = 1064;
    // var maxExtensionRlpHexLen = 4 + 2 + keyHexLen + 2 + 64;
    // check the root
    // check the depth
    // check the node types
    signal input state_root[64];



    // signal input address[20];
    signal input nonce;
    signal input balance;
    signal input storage_hash[32];
    signal input code_hash[32];

    // addressRlpPrefix:      2
    // addressRlpLength:      2
    // nonceRlpPrefix         2
    // nonce                  <= 0 nonce hash to be 0
    // balanceRlpPrefix       2
    // balance                <= 24
    // storageRootRlpPrefix   2
    // storageRoot            64
    // codeHashRlpPrefix      2
    // codeHash               64
    // 164
    // 164

    signal input account_rlp[164];
    signal input account_rlp_len;

    signal input account_proof[maxDepth][1064];
    signal input account_proof_length;
    signal input node_length[maxDepth];
    // signal input node_types[maxDepth];
    signal output a ;
    // 639225

    // log("in the circuit");
    // log(account_proof_length);
    // log(account_proof[0][0]);
    // log("in the circuit");
    // log(1 == 1);

    // log(node_types[0]);
    // log(node_types[1]);
    // log(node_types[2]);
    // log(node_types[3]);
    // log(node_length[0]);
    // log(node_length[1]);
    // log(node_length[2]);
    // log(node_length[3]);

    // signal nodehash[64];
    // signal nodehashlen;


    // component hash = KeccakOrLiteralHex(1064);
    // for (var i=0;i< 1064;i++){
    //     hash.in[i] <== account_proof[1][i];
    // }
    // hash.inLen <== node_length[1];

    // for (var i=0;i< 64;i++){
    //     nodehash[i] <== hash.out[i];
    // }
    
    // nodehashlen <== hash.outLen;


    // signal roothash
    component rootHash = KeccakOrLiteralHex(1064);
    for (var i=0;i< 1064;i++){
        rootHash.in[i] <== account_proof[0][i];
    }
    rootHash.inLen <== node_length[0];
    state_root === rootHash.out;

    // rlp check
    component isRlpValid = IsPaddedSubarray(1064, 164);
    isRlpValid.base <== account_proof[maxDepth -1];
    isRlpValid.sub <== account_rlp;
    isRlpValid.subRealLen <== account_rlp_len;
    isRlpValid.out === 1;

    // check to see if the keccak of each layer exist in the upper layer

    component subChecks[maxDepth];
    component nodeHash[maxDepth];
    for (var i=1; i < maxDepth; i++){
        nodeHash[i] = KeccakOrLiteralHex(1064);
        nodeHash[i].in <== account_proof[maxDepth - i];
        nodeHash[i].inLen <== node_length[maxDepth - i];

        subChecks[i] = IsSubarray(1064, 64);
        subChecks[i].base <== account_proof[maxDepth - i -1];
        subChecks[i].sub <== nodeHash[i].out;

        1 === subChecks[i].out;
    }


    // check account proof

    component rlpHexToByte = HexToBytes(164,82);
    rlpHexToByte.hexArray <== account_rlp;

    component rlpDecode = Rlp(82);
    rlpDecode.rlp <== rlpHexToByte.out;
    rlpDecode.rlpLen <== account_rlp_len / 2;

    nonce === rlpDecode.nonce;
    balance === rlpDecode.balance;
    storage_hash === rlpDecode.storageHash;
    code_hash === rlpDecode.codeHash;

    log(rlpDecode.nonce);
    log(rlpDecode.balance);
    log(rlpDecode.storageHash[0]);
    log(rlpDecode.storageHash[31]);
    log(rlpDecode.codeHash[0]);
    log(rlpDecode.codeHash[31]);


    a <== account_proof_length;



}


template HexToBytes(hexLen, bytesLen){
    signal input hexArray[hexLen];
    signal output out[bytesLen];

    hexLen/2 === bytesLen;

    var j = 0;
    for (var i=0; i< hexLen; i+=2){
        out[j] <== hexArray[i] * 16 + hexArray[i+1];
        j = j +1;
    }

}





component main = mpt(4);
// component main = HexToBytes(164,82);
// component main = SubarrayExists(25,5);
// component main = IsSubarray(7,3);
// 0x158730abb7b01c5c78b