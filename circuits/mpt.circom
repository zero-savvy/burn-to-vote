pragma circom 2.0.0;
include "./keccak.circom";
include "./utils.circom";
include "./rlp.circom";
include "./singleMux.circom";
include "circomlib/circuits/multiplexer.circom";

// add dynamic length

template Mpt(maxDepth){

    // var maxBranchRlpHexLen = 1064;

    // addressRlpPrefix:      2
    // addressRlpLength:      2
    // nonceRlpPrefix         2
    // nonce                  <= 0 nonce has to be 0
    // balanceRlpPrefix       2
    // balance                <= 24
    // storageRootRlpPrefix   2
    // storageRoot            64
    // codeHashRlpPrefix      2
    // codeHash               64


    signal input address[40];
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

    signal input leaf_nibbles;


    // check address nibs
    component getLeaf = Multiplexer(1064,maxDepth);
    getLeaf.inp <== account_proof;
    getLeaf.sel <== account_proof_length -1 ;

    component address_hash = KeccakAndPadHex(40);
	address_hash.in<== address;
    address_hash.inLen <== 40;


    component shift_nibs = VarShiftLeft(64,10);
    shift_nibs.in <== address_hash.out;
    shift_nibs.shift <== 64 - leaf_nibbles;

    component is_address_nibbs_in_leaf = IsPaddedSubarray(1064, 64);
    is_address_nibbs_in_leaf.base <== getLeaf.out;
    is_address_nibbs_in_leaf.sub <== shift_nibs.out;
    is_address_nibbs_in_leaf.subRealLen <== leaf_nibbles;
    is_address_nibbs_in_leaf.out === 1;


    // check state_root of the block
    component rootHash = KeccakOrLiteralHex(1064);
    rootHash.in <== account_proof[0];
    rootHash.inLen <== node_length[0];
    state_root === rootHash.out;

    // rlp existence check

    component isRlpValid = IsPaddedSubarray(1064, 164);
    isRlpValid.base <== getLeaf.out;
    isRlpValid.sub <== account_rlp;
    isRlpValid.subRealLen <== account_rlp_len;
    isRlpValid.out === 1;

    // check to see if the keccak of each layer exist in the upper layer

    component nodeHash[maxDepth];
    signal nodeHashes[maxDepth][64];
    for (var i=0; i < maxDepth; i++){
        nodeHash[i] = KeccakOrLiteralHex(1064);
        nodeHash[i].in <== account_proof[i];
        nodeHash[i].inLen <== node_length[i];

        nodeHashes[i] <== nodeHash[i].out;
    }


    component subChecks[maxDepth];
    for (var i=0; i < maxDepth-1; i++){
        subChecks[i] = IsSubarray(1064, 64);
        subChecks[i].base <== account_proof[i];
        subChecks[i].sub <== nodeHashes[i+1];

        1 === subChecks[i].out ;

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

// component main = Mpt(8);