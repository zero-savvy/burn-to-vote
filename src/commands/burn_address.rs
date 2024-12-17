use alloy::{
    hex::FromHex,
    primitives::{keccak256, Address, U8},
};
use structopt::StructOpt;

// 	BA = H(pkv  ||  vid || β  ||  pid || v)

#[derive(Debug, StructOpt)]
pub struct BurnAddress {
    private_key: String,
    ceremony_id: U8,
    blinding_factor: U8,
    personal_id: U8,
    vote: U8,
}

pub async fn burn_address(burn_address: BurnAddress) {
    let private_key_bytes = <[u8; 32]>::from_hex(burn_address.private_key).unwrap();
    let personal_id_bytes_bytes = burn_address.personal_id.to_be_bytes();
    let blinding_factor_bytes = burn_address.blinding_factor.to_be_bytes();
    let ceremony_id_bytes = burn_address.ceremony_id.to_be_bytes();
    let vote_bytes = burn_address.vote.to_be_bytes();

    let data = [
        private_key_bytes,
        ceremony_id_bytes,
        blinding_factor_bytes,
        personal_id_bytes_bytes,
        vote_bytes,
    ]
    .concat();

    let burn_address = Address::from_slice(&keccak256(data).0[12..]);

    println!("Burn Address: {:?}", burn_address)
}
