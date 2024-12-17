use structopt::StructOpt;
mod commands;
use alloy::{
    primitives::U8,
    hex::FromHex,
    signers::k256::SecretKey
};
#[derive(Debug, StructOpt)]
enum Opt {
    BurnAddress(BurnAddress),
    Burn,
    Vote,
    Verify,
}

#[derive(Debug, StructOpt)]
struct BurnAddress {
    private_key: String,
    ceremony_id: U8,
    blinding_factor: U8,
    personal_id: U8,
    vote: U8,
}
#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::BurnAddress(burn_address) => {
            let key_bytes : [u8; 32] = <[u8; 32]>::from_hex(burn_address.private_key).unwrap();
            let pk = SecretKey::from_bytes(&key_bytes.into()).unwrap();
            commands::burn_address::burn_address(
            pk,
            burn_address.ceremony_id,
            burn_address.blinding_factor,
            burn_address.personal_id,
            burn_address.vote,
             ).await},
        Opt::Burn => commands::burn::burn().await,
        Opt::Vote => commands::vote::vote().await,
        Opt::Verify => commands::verify::verify().await,
    }
}
