pragma circom  2.2.0;

include "circomlib/circuits/comparators.circom";
include "nullifier.circom";
include "mpt.circom";
include "burnAddress.circom";

template vote(maxDepth) {


    signal input address;
    signal input privateKey;
    signal input blinding_factor;
    signal input ceremonyID;
    signal input personalID;
    signal input vote;
    signal input nullifier;
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

    component burn_address = BurnAddress();
    burn_address.address <== address;
    burn_address.privateKey <== privateKey;
    burn_address.blinding_factor <== blinding_factor;
    burn_address.ceremonyID <== ceremonyID;
    burn_address.personalID <== personalID;
    burn_address.vote <== vote;

    signal generated_nullifier;
    component nullifier_generator = Nullifier();
    nullifier_generator.privateKey <== privateKey;
    nullifier_generator.blindingFactor <== blinding_factor;
    nullifier_generator.ceremonyID <== ceremonyID;
    generated_nullifier <== nullifier_generator.nullifier ;

    generated_nullifier === nullifier;

    component check_account = Mpt(maxDepth);
    check_account.nonce <== nonce;
    check_account.balance <== balance;
    check_account.storage_hash <== storage_hash;
    check_account.code_hash <== code_hash;
    check_account.state_root <== state_root;
    check_account.account_rlp <== account_rlp;
    check_account.account_rlp_len <== account_rlp_len;
    check_account.account_proof <== account_proof;
    check_account.account_proof_length <== account_proof_length;
    check_account.node_length <== node_length;

}

component main = vote(8);