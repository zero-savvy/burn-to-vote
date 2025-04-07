pragma circom 2.0.0;
// include "circuits/circom-ecdsa/circuits/vocdoni-keccak/keccak.circom";
include "./keccak.circom";
// include "circomlib/circuits/comparators.circom";
template IsZero() {
    signal input in;
    signal output out;

    signal inv;

    inv <-- in!=0 ? 1/in : 0;

    out <== -in*inv +1;
    in*out === 0;
}

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
    // signal input nonce;
    // signal input balance;
    // signal input storage_hash;
    // signal input code_hash;

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

    signal account_rlp[164];
    signal account_rlp_len;

    signal input account_proof[maxDepth][1064];
    signal input account_proof_length;
    signal input node_length[maxDepth];
    // signal input node_types[maxDepth];
    signal output a ;

    log("in the circuit");
    log(account_proof_length);
    log(account_proof[0][0]);
    log("in the circuit");
    log(1 == 1);

    // log(node_types[0]);
    // log(node_types[1]);
    // log(node_types[2]);
    // log(node_types[3]);
    log(node_length[0]);
    log(node_length[1]);
    log(node_length[2]);
    log(node_length[3]);

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
    // log("circuit node len ");
    // log(node_length[0]);
    // log(account_proof[0][0]);
    // log(account_proof[0][node_length[0]-1]);
    // log("root hash daya");

    // log(node0hash.out[0]);
    // log(node0hash.out[1]);
    // log(node0hash.out[2]);
    // log(node0hash.out[3]);


    
    // nodehashlen <== node0ha/sh.outLen;

    // log("circuit data ");
    // log(nodehashlen);
    // log("circuit data ");
    // log(nodehash[0]);
    // log(nodehash[1]);
    // log(nodehash[2]);
    // log(nodehash[3]);

// 8
// 1
// 10
// 10


    a <== account_proof_length;

// 0xf9017180a0ab8cdb808c8303bb61fb48e276217be9770fa83ecf3f90f2234d558885f5abf18080a01c6bd4f83b6c6efe02fedfcfba9f7feec2bd930b818e082fd970ca38b9b97af8a081aaaaa996afd124cf58fd3f38de5a059f1905d
}

// aggregated sum verification method
template SubarrayExists(n, m) {
    signal input base[n];
    signal input sub[m];
    signal output exists;

    var isLenValid = 1;
    if (n < m){
        isLenValid = 0;
    }


    var sub_sum = 0;
    for (var j=0; j<m; j++){
        sub_sum = (sub[j] * (2 ** (m-j-1))) + sub_sum;
    }
    // var isValid = 0;
    signal isValid[n-m+1];
    component isSumEq[n-m+1];
    for (var i=0; i<n-m+1; i++){
        var sum = 0 ;
        for (var j=0; j<m; j++){
            sum = base[i+j] * (2 ** (m-1-j)) + sum;

        }
        isSumEq[i] = IsEqual();
        isSumEq[i].in[0] <== sum;
        isSumEq[i].in[1] <== sub_sum;
        // isValid = isValid + isSumEq[i].out;
        if(i == 0) {
            isValid[i] <== isSumEq[i].out;
        } else {
            isValid[i] <== isValid[i-1] + isSumEq[i].out;
        }

    } 
// fix this
    component isOne = IsEqual();
    isOne.in[0] <== isValid[n-m];
    isOne.in[1] <== 1;
    exists <== isOne.out * isLenValid;
    log(exists);


}




component main = mpt(4);
// component main = SubarrayExists(25,5);
// 0x158730abb7b01c5c78b14f706b9701f9f2cf58fd57803b8b07080b086f090807
// account_proof: [Bytes(0xf9017180a0ab8cdb808c8303bb61fb48e276217be9770fa83ecf3f90f2234d558885f5abf18080a01c6bd4f83b6c6efe02fedfcfba9f7feec2bd930b818e082fd970ca38b9b97af8a081aaaaa996afd124cf58fd3f38de5a059f1905d3a6b2c2090bcf89630b8d08a8a09831837545c9d1f85528841634099d44272f76c73c291d0f652d65ef339a8aa280a063ffd23f38fd5886da541f64fc4ce465779d7951e6a8ae7a3fe4fa7890ed38f2a0c326f61dd1e74e037d4db73aede5642260bf92869081753bbace550a73989aeda06301b39b2ea8a44df8b0356120db64b788e71f52e1d7a6309d0d2e5b86fee7cb80a029087b3ba8c5129e161e2cb956640f4d8e31a35f3f133c19a1044993def98b61a0d53feb0a699dd7b3dda91861cd28f4e6c420618668edc681b7ad88f7da65430da061f17d95797b56190d7cccbdcc2b8bbff3744dde71777d706e2078e2192aaad2a0073e08ab6f69eaf4f99afa0fe78e53160ab3a61b84993b3cb58c2607c517782980), Bytes(0xf87180a0b0da6320e7908e965247051d0c7a60c24944e3c56a8d31d681c0f1c046b9219e80a09a1290a78ca21cff91dca17280fb1660aef8803c5b4229ebaf0dcd066ded426380808080808080a0e61e567237b49c44d8f906ceea49027260b4010c10a547b38d8b131b9d3b6f848080808080), Bytes(0xf851808080808080a0b61385b0ab3f5cdf75a9dc322edebc6a5223a5aec9561a1547478e023f41813080808080808080a077277c9893e4486f353ea59f3b0e9a6fa4e3e705222aa1d879ae50ea341aa5688080), Bytes(0xf8719f33b5f4c00fb61f3aa209d0cdd0d5fa9762641816fc46ac912ef70299939646b84ff84d808901158e460913d00000a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a0c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470)],
