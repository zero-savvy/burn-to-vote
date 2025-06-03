use crate::circuits::burn_address_c::*;
// use crate::circuits::Circuit;
use crate::utils::{config::*, fr_repr_to_bytes, u256_to_fp};
use ethers::types::Address;
use ff::PrimeField;
use log::info;
use poseidon_rs::{Fr, FrRepr, Poseidon};
use primitive_types::U256;
use structopt::StructOpt;
#[derive(Debug, StructOpt, Clone)]
pub struct BurnAddress {
    pub private_key: String,
    pub ceremony_id: u64,
    pub blinding_factor: u64,
    pub vote: u64,
}

pub async fn burn_address(
    config: Config,
    private_key: String,
    blinding_factor: u64,
    vote: u64,
) -> (BurnAddressCircuit, Address) {
    info!("Genrating burn address ...");

    let private_key = U256::from_str_radix(&private_key.clone(), 16).unwrap();

    let private_key_fp = u256_to_fp(private_key);

    let ceremony_id_fp = Fr::from_repr(FrRepr::from(config.ceremony_id.unwrap())).unwrap();
    let blinding_factor_fp = Fr::from_repr(FrRepr::from(blinding_factor)).unwrap();
    let random_secret_fp = Fr::from_repr(FrRepr::from(blinding_factor)).unwrap();
    let vote_fp = Fr::from_repr(FrRepr::from(vote)).unwrap();

    let input: Vec<Fr> = vec![
        private_key_fp,
        ceremony_id_fp,
        blinding_factor_fp,
        random_secret_fp,
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
        config.ceremony_id.unwrap(),
        blinding_factor,
        vote,
    );

    (circuit, address)
}

