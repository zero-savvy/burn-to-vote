use ff::PrimeField;
use poseidon_rs::Fr;
use structopt::StructOpt;
mod circuits;
mod commands;
mod utils;
use utils::u256_to_fp;
use alloy::primitives::{address, Address};
use commands::burn::Burn;
use commands::burn_address::BurnAddress;
use commands::demo::DemoData;
use commands::merkle_tree::UserIndex;
use commands::nullifier::Nullifier;
use commands::vote::Vote;
use env_logger::Env;
use ethers::providers::{Http, Provider};
use std::fs::{self, File};
use std::path::Path;
type PrimitiveU256 = primitive_types::U256;


// TO DO:
// error handling
// add mpt dynamic length
// add address to mpt
// dynamic config and chain
// prepare vote contract
// check variables name convention
// add contract tests
// deploy contract

#[derive(Debug, StructOpt)]
enum Opt {
    BurnAddress(BurnAddress),
    Burn(Burn),
    Nullifier(Nullifier),
    // Vote(Vote),
    Verify,
    GenerateTree,
    GenerateProof(UserIndex),
    Demo(DemoData),
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let provider: Provider<Http> = Provider::<Http>::try_from("http://localhost:8545/")
        .unwrap()
        .clone();

    let path = Path::new("data/whitelist.json");
    let file = File::open(path).unwrap();
    let mut whitelist: Vec<String> = serde_json::from_reader(file).unwrap();
    let mut whitelist_fr = whitelist.iter().map(|x| {
        let num = PrimitiveU256::from_str_radix(x,16).expect("Error: failed to get u256 from hash.");
        u256_to_fp(num)

    }).collect();

    let opt = Opt::from_args();
    match opt {
        Opt::BurnAddress(burn_address) => {
            commands::burn_address::burn_address(burn_address).await;
        }
        Opt::Burn(burn_data) => {
            commands::burn::burn(burn_data, provider).await;
        }
        Opt::Nullifier(nullifier) => {
            commands::nullifier::generate_nullifier(nullifier);
        }
        // Opt::Vote(vote_data) => {
        //     commands::vote::vote(vote_data, provider).await;
        // }
        Opt::GenerateTree => {
            commands::merkle_tree::generate_tree(&mut whitelist_fr).await;
        }
        Opt::GenerateProof(user_index) => {
            let tree = commands::merkle_tree::generate_tree(&mut whitelist_fr).await;
            commands::merkle_tree::generate_proof(&tree, user_index.index).await;
        }
        Opt::Verify => {
            commands::verify::verify().await;
        }
        Opt::Demo(demo_data) => {
            commands::demo::demo(demo_data, provider).await;
        }
    }
}
