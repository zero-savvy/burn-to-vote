// bid
// 1. initiare auction starts with the token distribiutions
// contract is deployes after the start of the project 
// and after the last block of voting since we need the state root of the last blick

use log::info;
use structopt::StructOpt;

use ethers::providers::{Http, Middleware, Provider};
use ff::PrimeField;
use poseidon_rs::{Fr, FrRepr, Poseidon};
use primitive_types::U256 as PrimitiveU256;

use crate::{
    circuits::{bid_c::BidCircuit, Circuit},
    commands::{
        burn::burn,
        burn_address::burn_address,
        merkle_tree::{generate_proof, generate_tree},
        nullifier::generate_nullifier,
    },
    utils::{account::prepare_mpt_data, config::*, u256_to_fp},
};

#[derive(Debug, StructOpt, Clone)]
pub struct Bid {
    #[structopt(long)]
    pub ceremony_id: Option<u64>,
    #[structopt(long)]
    pub voting_block: u64,
    #[structopt(long)]
    pub amount: PrimitiveU256,
    #[structopt(long)]
    pub bid: u64,
    #[structopt(long)]
    pub private_key: String,
}

pub async fn bid(config: &mut Config, bid_data: Bid) -> Result<(), Box<dyn std::error::Error>> {
    info!("initiate voting ..");

    let provider: Provider<Http> = Provider::<Http>::try_from(config.network.url())?.clone();

    let blinding_factor = rand::random::<u64>();

    let (burn_address_data, burn_address, block_hash) = burn_address(
        config.clone(),
        bid_data.private_key.clone(),
        blinding_factor,
        bid_data.voting_block,
        bid_data.bid,
    )
    .await;

    let nullifier = generate_nullifier(
        config.clone(),
        blinding_factor,
        bid_data.private_key.clone(),
    )
    .await;

    let (_, provider) = burn(
        provider,
        burn_address,
        bid_data.amount,
        bid_data.private_key.clone(),
    )
    .await;

    let mpt_data = prepare_mpt_data(burn_address, provider).await;

    let secret: PrimitiveU256 = PrimitiveU256::from_str_radix(&bid_data.private_key, 16)
        .expect("Error: failed to get u256 from secret.");
    let random_secret_fr = Fr::from_repr(FrRepr::from(blinding_factor))
        .expect("Error: failed to unwrap random secret Fr.");
    let index = config.white_list.len() - 1;
    let serct_fr = u256_to_fp(secret);
    let leaf_data = [serct_fr, random_secret_fr];
    let leaf_hasher = Poseidon::new();
    let leaf = leaf_hasher
        .hash(leaf_data.to_vec())
        .expect("Error: Failed to hash leaf data.");
    let mut white_list: Vec<Fr> = Vec::new();
    for i in 0 .. config.white_list.len() {
        let str2uint = PrimitiveU256::from_str_radix(&config.white_list[i], 16)
        .expect("Error: failed to get u256 from whitelist address.");
        white_list[i] = u256_to_fp(str2uint);
    };
    let tree = generate_tree(&mut white_list).await;
    let merkle_tree_proof = generate_proof(&tree, index).await;

    let circuit = BidCircuit::new(
        burn_address_data.address,
        secret,
        blinding_factor,
        burn_address_data.ceremony_id,
        block_hash,
        burn_address_data.random_secret,
        burn_address_data.action_value,
        config.min_bid.unwrap(),
        nullifier,
        mpt_data.nonce,
        mpt_data.balance,
        mpt_data.code_hash,
        mpt_data.storage_hash,
        mpt_data.state_root,
        mpt_data.account_rlp,
        mpt_data.account_rlp_len,
        mpt_data.account_proof,
        mpt_data.account_proof_length,
        mpt_data.node_length,
        mpt_data.leaf_nibbles,
        merkle_tree_proof.root,
        merkle_tree_proof.leaf,
        merkle_tree_proof.pathElements,
        merkle_tree_proof.pathIndices,
    );

    info!("updated config: {:?}", config);
    info!("Auction circuit: ");
    let inputs = circuit.format_inputs()?;
    circuit.generate_input_file(inputs)?;
    circuit.generate_witness()?;
    circuit.setup_zkey()?;
    circuit.generate_proof()?;
    circuit.setup_vkey()?;
    circuit.verify_proof()?;
    circuit.generate_verifier()
}
