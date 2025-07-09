use log::info;
use structopt::StructOpt;

use ethers::providers::{Http, Provider};
use ff::PrimeField;
use poseidon_rs::{Fr, FrRepr, Poseidon};
use primitive_types::U256 as PrimitiveU256;

// TODO: BOUND VOTERS TO ONE VOTE
use crate::{
    circuits::{vote_c::VoteCircuit, Circuit},
    commands::{
        burn::burn,
        burn_address::burn_address,
        merkle_tree::{generate_proof, generate_tree},
        nullifier::generate_nullifier,
    },
    utils::{account::prepare_mpt_data, config::*, u256_to_fp},
};

#[derive(Debug, StructOpt, Clone)]
pub struct Vote {
    #[structopt(long)]
    pub ceremony_id: Option<u64>,
    #[structopt(long)]
    pub voting_block: u64,
    #[structopt(long)]
    pub amount: PrimitiveU256,
    #[structopt(long)]
    pub vote: u64,
    #[structopt(long)]
    pub revote: u64,
    #[structopt(long)]
    pub private_key: String,
}

pub async fn vote(config: &mut Config, vote_data: Vote) -> Result<(), Box<dyn std::error::Error>> {
    info!("initiate voting ..");

    let provider: Provider<Http> = Provider::<Http>::try_from(config.network.url())?.clone();

    let blinding_factor = rand::random::<u64>();

    let (burn_address_data, burn_address) = burn_address(
        config.clone(),
        vote_data.private_key.clone(),
        blinding_factor,
        vote_data.voting_block,
        vote_data.vote,
    )
    .await;

    let nullifier = generate_nullifier(
        config.clone(),
        blinding_factor,
        vote_data.private_key.clone(),
    )
    .await;

    let (_, provider) = burn(
        provider,
        burn_address,
        vote_data.amount,
        vote_data.private_key.clone(),
    )
    .await;

    let mpt_data = prepare_mpt_data(burn_address, provider).await;

    let secret: PrimitiveU256 = PrimitiveU256::from_str_radix(&vote_data.private_key, 16)
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

    let circuit = VoteCircuit::new(
        burn_address_data.address,
        secret,
        blinding_factor,
        burn_address_data.ceremony_id,
        burn_address_data.random_secret,
        burn_address_data.vote,
        vote_data.revote,
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

    match vote_data.vote {
        1 => config.yesVotes = Some(config.yesVotes.unwrap_or(0) + 1),
        0 => config.noVotes = Some(config.noVotes.unwrap_or(0) + 1),
        _ => {
            return Err("invalid vote value".into());
        }
    }
    info!("updated config: {:?}", config);
    info!("VOTE circuit: ");
    let inputs = circuit.format_inputs()?;
    circuit.generate_input_file(inputs)?;
    circuit.generate_witness()?;
    circuit.setup_zkey()?;
    circuit.generate_proof()?;
    circuit.setup_vkey()?;
    circuit.verify_proof()?;
    circuit.generate_verifier()
}
