use crate::circuits::nullifier_c::*;
use crate::circuits::Circuit;
use crate::utils::u256_to_fp;
use ff::PrimeField;
use log::info;
use poseidon_rs::{Fr, FrRepr, Poseidon};
use primitive_types::U256;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Nullifier {
    pub private_key: String,
    pub ceremony_id: u64,
    pub blinding_factor: u64,
}

pub fn generate_nullifier(data: Nullifier) -> String {
    let private_key = U256::from_str_radix(&data.private_key, 16).unwrap();
    let private_key_fp = u256_to_fp(private_key);

    let blinding_factor_fp = Fr::from_repr(FrRepr::from(data.blinding_factor)).unwrap();
    let ceremony_id_fp = Fr::from_repr(FrRepr::from(data.ceremony_id)).unwrap();

    let input: Vec<Fr> = vec![private_key_fp, ceremony_id_fp, blinding_factor_fp];

    let poseidon = Poseidon::new();
    let hash = poseidon.hash(input).unwrap();

    let hash_string = hash.into_repr().to_string();
    
    hash_string
}


