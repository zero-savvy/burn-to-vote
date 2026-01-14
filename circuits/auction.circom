pragma circom  2.2.0;

include "circomlib/circuits/comparators.circom";
include "nullifier.circom";
include "mpt.circom";
include "merkleTree.circom";
include "burnAddress.circom";

template bid(maxDepth) {

    signal input address;
    signal input nullifier;

    signal input secret;
    signal input random_secret;
    signal input blinding_factor;
    signal input ceremonyID;
    signal input votingBlockHash;
    
    signal input nonce;
    signal input balance;
    signal input storage_hash[32];
    signal input code_hash[32];

    signal input state_root[64];
    signal output state_root_hex;

    signal input account_rlp[164];
    signal input account_rlp_len;

    signal input account_proof[maxDepth][1064];
    signal input account_proof_length;
    signal input node_length[maxDepth];
    signal input leaf_nibbles;

    signal input bidMin;
    signal input bid;


    // security checks

    component bidCheck = LessEqThan(252);
    bidCheck.in[0] <== bidMin;
    bidCheck.in[1] <== bid;
    bidCheck.out === 1;


    component rlp_len_check = LessEqThan(8);
    rlp_len_check.in[0] <== account_rlp_len;
    rlp_len_check.in[1] <== 164;

    rlp_len_check.out === 1;


    component mpt_proof_length_check = LessEqThan(4);
    mpt_proof_length_check.in[0] <== account_proof_length;
    mpt_proof_length_check.in[1] <== maxDepth;

    mpt_proof_length_check.out === 1;


    component address_nibble_length_check = LessEqThan(7);
    address_nibble_length_check.in[0] <== leaf_nibbles;
    address_nibble_length_check.in[1] <== 64;

    address_nibble_length_check.out === 1;

    nonce === 0;


    component balance_check = Num2Bits(256);
    balance_check.in <== balance;

    component secret_check = Num2Bits(256);
    secret_check.in <== secret;

    component blinding_factor_check = Num2Bits(64);
    blinding_factor_check.in <== blinding_factor;

    component ceremony_id_check = Num2Bits(64);
    ceremony_id_check.in <== ceremonyID;    
    
    component random_secret_check = Num2Bits(64);
    random_secret_check.in <== random_secret;



    log("burn address check ... ");
    component burn_address = BurnAddress();
    burn_address.secret <== secret;
    burn_address.blinding_factor <== blinding_factor;
    burn_address.ceremonyID <== ceremonyID;
    burn_address.random_secret <== random_secret;
    burn_address.action_value <== bid;
    burn_address.votingBlockHash <== votingBlockHash;

    address === burn_address.address;


    log("mt check ... ");

    signal input mt_root;
    signal input mt_leaf;
    signal input mt_pathElements[2];
    signal input mt_pathIndices[2];

    component merkle_tree_inclusion = MerkleTreeChecker(2);
    merkle_tree_inclusion.leaf <== mt_leaf;
    merkle_tree_inclusion.pathElements <== mt_pathElements;
    merkle_tree_inclusion.pathIndices <== mt_pathIndices;

    mt_root === merkle_tree_inclusion.root ;
    mt_leaf === burn_address.secret_commitment;




    log("nullifier check ... ");

    component nullifier_generator = Nullifier();
    nullifier_generator.secret <== secret;
    nullifier_generator.blindingFactor <== blinding_factor;
    nullifier_generator.ceremonyID <== ceremonyID;

    nullifier === nullifier_generator.nullifier ;


    log("mpt check ... ");


    component n2b_address = Num2Bits(256);
    n2b_address.in <== address;

    component addr_bit2num = Bits2Num(160);
    for (var i = 0; i < 160; i++) {
        addr_bit2num.in[i] <== n2b_address.out[i];
    }

    signal hex_address[40];
    component h2d = HexToDigits();
    h2d.addr <== addr_bit2num.out;
    hex_address <== h2d.digits; 


    component check_account = Mpt(maxDepth);

    check_account.address <== hex_address;
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
    check_account.leaf_nibbles <== leaf_nibbles;

    component root_hex  = CombineHex(64);
    root_hex.nibbles <== state_root;
    state_root_hex <== root_hex.hexNumber;

    log("state_root_hex");
    log(state_root_hex);


}

component main{public[ceremonyID, nullifier, bid, bidMin, mt_root, address]}  = bid(8);


// public data
// 0 => state_root
// 1 => nullifier
// 2 => ceremonyID
// 3 => bid
// 4 => bidMin
// 5 => mt_root