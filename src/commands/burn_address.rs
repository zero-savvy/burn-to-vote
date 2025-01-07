use super::utils::{fr_repr_to_bytes, u256_to_fp};
use alloy::primitives::Address;
use ff::PrimeField;
use poseidon_rs::{Fr, FrRepr, Poseidon};
use primitive_types::U256;
use std::fs::File;
use std::io::prelude::*;
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

    let inputs_json = format!(
        "{{ \"address\": \"{}\",
        \"privateKey\": \"{}\",
        \"blinding_factor\": \"{}\",
        \"ceremonyID\": \"{}\",
        \"personalID\": \"{}\",
        \"vote\": \"{}\" }}",
        U256::from_str_radix(&rep_str[2..], 16).unwrap().to_string(),
        private_key.to_string(),
        serde_json::to_string(&blinding_factor).unwrap(),
        serde_json::to_string(&burn_address.ceremony_id).unwrap(),
        serde_json::to_string(&burn_address.personal_id).unwrap(),
        serde_json::to_string(&burn_address.vote).unwrap()
    );

    let mut f = File::create("inputs/burn_address.json").unwrap();
    f.write_all(inputs_json.as_bytes()).unwrap();

    address
}
