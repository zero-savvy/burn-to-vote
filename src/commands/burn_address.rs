use ff::PrimeField;
use poseidon_rs::{Fr, FrRepr, Poseidon};
use structopt::StructOpt;
// 	BA = H(pkv  ||  vid || β  ||  pid || v)

#[derive(Debug, StructOpt)]
pub struct BurnAddress {
    private_key: String,
    ceremony_id: u64,
    blinding_factor: u64,
    personal_id: u64,
    vote: u64,
}

pub async fn burn_address(burn_address: BurnAddress) -> String {
    let private_key = Fr::from_str(&burn_address.private_key).unwrap();
    let ceremony_id = Fr::from_repr(FrRepr::from(burn_address.ceremony_id)).unwrap();
    let blinding_factor = Fr::from_repr(FrRepr::from(burn_address.blinding_factor)).unwrap();
    let personal_id = Fr::from_repr(FrRepr::from(burn_address.personal_id)).unwrap();
    let vote = Fr::from_repr(FrRepr::from(burn_address.vote)).unwrap();

    let poseidon = Poseidon::new();
    let input: Vec<Fr> = vec![private_key, ceremony_id, blinding_factor, personal_id, vote];
    poseidon.hash(input).unwrap().to_string()
}
