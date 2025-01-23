use crate::circuits::burn_address::*;
use crate::circuits::Circuit;
use crate::utils::general::{fr_repr_to_bytes, u256_to_fp};
use alloy::primitives::Address;
use ff::PrimeField;
use log::info;
use poseidon_rs::{Fr, FrRepr, Poseidon};
use primitive_types::U256;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct BurnAddress {
    private_key: String,
    ceremony_id: u64,
    personal_id: u64,
    vote: u64,
}

pub async fn burn_address(burn_address: BurnAddress) -> Address {
    let blinding_factor = rand::random::<u64>();
    let private_key = U256::from_str_radix(&burn_address.private_key, 16).unwrap();

    let private_key_fp = u256_to_fp(private_key);

    let ceremony_id_fp = Fr::from_repr(FrRepr::from(burn_address.ceremony_id)).unwrap();
    let blinding_factor_fp = Fr::from_repr(FrRepr::from(blinding_factor)).unwrap();
    let personal_id_fp = Fr::from_repr(FrRepr::from(burn_address.personal_id)).unwrap();
    let vote_fp = Fr::from_repr(FrRepr::from(burn_address.vote)).unwrap();

    let input: Vec<Fr> = vec![
        private_key_fp,
        ceremony_id_fp,
        blinding_factor_fp,
        personal_id_fp,
        vote_fp,
    ];

    let poseidon = Poseidon::new();
    let hash = poseidon.hash(input).unwrap();
    let rep = hash.into_repr();
    let rep_str = hash.into_repr().to_string();
    let bytes = fr_repr_to_bytes(&rep);
    let address_bytes = &bytes[12..];
    let address = Address::from_slice(address_bytes);

    let circuit = BurnAddressCircuit::new(
        rep_str.clone(),
        private_key,
        blinding_factor,
        burn_address.ceremony_id,
        burn_address.personal_id,
        burn_address.vote,
    );

    info!("Burn address circuit: ");
    let inputs = circuit.format_inputs().unwrap();
    circuit.generate_input_file(inputs).unwrap();
    circuit.generate_witness().unwrap();
    circuit.setup_zkey().unwrap();
    circuit.generate_proof().unwrap();
    circuit.setup_vkey().unwrap();
    circuit.verify_proof().unwrap();
    address
}
