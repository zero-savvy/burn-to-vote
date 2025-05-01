use super::{
    burn::{burn, Burn},
    burn_address::{burn_address, BurnAddress},
};
use crate::{
    circuits::Circuit,
    commands::nullifier::{self, generate_nullifier, Nullifier},
    utils::account::prepare_mpt_data,
};
use chrono::Local;
use ethers::providers::{Http, Provider};
use log::info;
// use primitive_types::U256;
use structopt::StructOpt;
type PrimitiveU256 = primitive_types::U256;
type EthersU256 = ethers::types::U256;

use super::super::utils::account;
use crate::circuits::vote_c::VoteCircuit;

#[derive(Debug, StructOpt)]
pub struct Vote {
    pub private_key: String,
    pub ceremony_id: u64,
    pub personal_id: u64,
    pub vote: u64,
    pub amount: PrimitiveU256,
}

pub async fn vote(vote_data: Vote, provider: Provider<Http>) -> String {
    let burn_address_data = BurnAddress {
        private_key: vote_data.private_key.clone(),
        ceremony_id: vote_data.ceremony_id.clone(),
        personal_id: vote_data.personal_id.clone(),
        vote: vote_data.vote.clone(),
    };

    let (burn_address_data, burn_address) = burn_address(burn_address_data).await;

    let burn_data = Burn {
        private_key: vote_data.private_key.clone(),
        burn_address: burn_address,
        amount: vote_data.amount,
    };

    let (_, provider) = burn(burn_data, provider).await;

    let nullifier_data = Nullifier {
        private_key: vote_data.private_key.clone(),
        ceremony_id: vote_data.ceremony_id.clone(),
        blinding_factor: burn_address_data.blinding_factor.clone(),
    };

    let nullifier = generate_nullifier(nullifier_data);

    let mpt_data = prepare_mpt_data(burn_address, provider).await;

    type EthersU256 = ethers::types::U256;
    let private_key: EthersU256 =
        EthersU256::from_str_radix(&vote_data.private_key.clone(), 16).unwrap();

    let circuit = VoteCircuit::new(
        burn_address_data.address,
        private_key,
        burn_address_data.blinding_factor,
        burn_address_data.ceremony_id,
        burn_address_data.personal_id,
        burn_address_data.vote,
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
    );

    info!("account_proof_length: {:?}", mpt_data.account_proof_length);
    info!("VOTE circuit: ");
    let inputs = circuit.format_inputs().unwrap();
    circuit.generate_input_file(inputs).unwrap();
    circuit.generate_witness().unwrap();
    circuit.setup_zkey().unwrap();
    circuit.generate_proof().unwrap();
    circuit.setup_vkey().unwrap();
    circuit.verify_proof().unwrap();

    "".to_string()
}
