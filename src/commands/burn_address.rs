use alloy::primitives::Address;
use ff::PrimeField;
use poseidon_rs::{Fr, FrRepr, Poseidon};
use structopt::StructOpt;
use sha3::{Digest, Keccak256};
// 	BA = H(pkv  ||  vid || β  ||  pid || v)

#[derive(Debug, StructOpt)]
pub struct BurnAddress {
    private_key: String,
    ceremony_id: u64,
    blinding_factor: u64,
    personal_id: u64,
    vote: u64,
}

fn fr_repr_to_bytes(fr_repr: &FrRepr) -> [u8; 32] {
    unsafe { std::mem::transmute(*fr_repr) } 
}


pub async fn burn_address(burn_address: BurnAddress) -> Address {
    let private_key = Fr::from_str(&burn_address.private_key).unwrap();
    let ceremony_id = Fr::from_repr(FrRepr::from(burn_address.ceremony_id)).unwrap();
    let blinding_factor = Fr::from_repr(FrRepr::from(burn_address.blinding_factor)).unwrap();
    let personal_id = Fr::from_repr(FrRepr::from(burn_address.personal_id)).unwrap();
    let vote = Fr::from_repr(FrRepr::from(burn_address.vote)).unwrap();

    let input: Vec<Fr> = vec![private_key, ceremony_id, blinding_factor, personal_id, vote];

    let poseidon = Poseidon::new();
    let hash = poseidon.hash(input).unwrap();
    let rep = hash.into_repr();

    let serialized_hash = fr_repr_to_bytes(&rep);
    let keccak_hash = Keccak256::digest(&serialized_hash);
    let address_bytes:[u8;20] = keccak_hash[12..32].try_into().unwrap();
    Address::from(address_bytes)
    
}
