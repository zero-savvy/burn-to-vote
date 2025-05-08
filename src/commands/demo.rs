use super::merkle_tree::{generate_proof, generate_tree};
use super::vote::{vote, Vote};
use ff::PrimeField;
use ethers::prelude::*;
use ethers::providers::{Http, Middleware, Provider};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
type PrimitiveU256 = primitive_types::U256;
use log::info;
use std::error::Error;
use std::process::Command;
use structopt::StructOpt;
use poseidon_rs::{Fr, FrRepr, Poseidon};
use crate::utils::u256_to_fp;

#[derive(Debug, StructOpt)]

pub struct DemoData {
    pk: String,
}

pub async fn demo(demo_data: DemoData, provider: Provider<Http>) {
    info!("Voting demo ...");


    let chain_id = provider.get_chainid().await.unwrap();

    let wallet = demo_data
        .pk
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(chain_id.as_u64());

    // secret data prepration
    let secret = PrimitiveU256::from_str_radix(&demo_data.pk, 16).expect("Error: failed to get u256 from secret.");
    let random_secret = rand::random::<u64>();
    let serct_fr = u256_to_fp(secret);
    let random_secret_fr = Fr::from_repr(FrRepr::from(random_secret)).expect("Error: failed to unwrap random secret Fr.");
    let leaf_data = [serct_fr, random_secret_fr];
    let leaf_hasher = Poseidon::new();
    let leaf = leaf_hasher.hash(leaf_data.to_vec()).expect("Error: Failed to hash leaf data.");

    info!("Voting address: {:?}", wallet.address());
    info!("Generating whitelist tree ...");

    let path = Path::new("data/whitelist.json");

    let data = fs::read_to_string(path).expect("Error: failed to read the file");

    let mut deserialize_data: Value =
        serde_json::from_str(&data).expect("Error: failed to Deserialize the data");
    let mut index = 0;
    if let Value::Array(ref mut arr) = deserialize_data {
        arr.pop();
        arr.push(json!(leaf.into_repr().to_string()));
        index = arr.len() - 1;
    }

    let serialize_data =
        serde_json::to_string_pretty(&deserialize_data).expect("Error: failed to serialize data");

    fs::write(path, serialize_data).expect("Error: failed to write the data.");

    let white_list_data = fs::read_to_string(path).expect("Error: failed to read the file");
    let mut leaves_strings: Vec<String> = serde_json::from_str(&white_list_data).expect("Error reding addresses.");
    let mut leaves_fr = leaves_strings.iter().map(|x| {
        let num = PrimitiveU256::from_str_radix(x,16).expect("Error: failed to get u256 from hash.");
        u256_to_fp(num)

    }).collect();



    let tree = generate_tree(&mut leaves_fr).await;
    info!("Whitelist tree generated successfully.");

    info!("Generating whitelist proof ...");

    let merkle_tree_data = generate_proof(&tree, index).await;

    info!("Whitelist proof generated successfully");

    info!("Compiling vote_circuit ...");
    info!("This could take a while ...");

    run_command("make vote").expect("Error: Failed to compile vote circuit.");
    info!("Vote_circuit compiled successfully.");

    let vote_data = Vote {
        private_key: demo_data.pk,
        random_secret: random_secret,
        ceremony_id: rand::random::<u64>(),
        vote: 0,
        amount: PrimitiveU256::from(1)
    };

    vote(vote_data, provider, merkle_tree_data).await;

}

fn run_command(command: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("sh").arg("-c").arg(command).output()?;
    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Command failed: {}", stdout),
        )));
    }
    Ok(())
}
