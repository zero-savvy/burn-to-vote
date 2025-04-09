pragma circom 2.0.0;
include "./keccak.circom";
include "./utils.circom";
include "./rlp.circom";


template mpt(maxDepth){

    // var maxBranchRlpHexLen = 1064;

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

    signal input nonce;
    signal input balance;
    signal input storage_hash[32];
    signal input code_hash[32];

    signal input state_root[64];

    signal input account_rlp[164];
    signal input account_rlp_len;

    signal input account_proof[maxDepth][1064];
    signal input account_proof_length;
    signal input node_length[maxDepth];


    // signal roothash
    component rootHash = KeccakOrLiteralHex(1064);
    for (var i=0;i< 1064;i++){
        rootHash.in[i] <== account_proof[0][i];
    }
    rootHash.inLen <== node_length[0];
    state_root === rootHash.out;

    // rlp existence check
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

}



component main = mpt(4);
