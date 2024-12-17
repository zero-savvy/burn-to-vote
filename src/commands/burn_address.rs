use alloy::{
    primitives::{keccak256, Address, U8},
    signers::k256::SecretKey
};

// 	BA = H(pkv  ||  vid || β  ||  pid || v)

pub async fn burn_address(
    private_key: SecretKey,
    ceremony_id: U8,
    blinding_factor: U8,
    personal_id: U8,
    vote: U8,
) {
    let private_key_bytes = private_key.to_bytes().into();
    let personal_id_bytes_bytes = personal_id.to_be_bytes();
    let blinding_factor_bytes = blinding_factor.to_be_bytes();
    let ceremony_id_bytes = ceremony_id.to_be_bytes();
    let vote_bytes = vote.to_be_bytes();

    let data = [private_key_bytes, ceremony_id_bytes, blinding_factor_bytes, personal_id_bytes_bytes, vote_bytes].concat();

    let burn_address = Address::from_slice(
        &keccak256(data).0[12..],
    );

    println!("Burn Address: {:?}", burn_address)
}

