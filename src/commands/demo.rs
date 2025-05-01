use super::merkle_tree::{generate_proof, generate_tree};
use super::vote::{vote, Vote};
use ::rand::seq::index;
use ethers::prelude::*;
use ethers::providers::{Http, Middleware, Provider};
use serde::Deserializer;
use serde_json::{json, Value};
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
type PrimitiveU256 = primitive_types::U256;
use alloy::primitives::{address, Address};
use log::info;
use std::error::Error;
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]

pub struct DemoData {
    pk: String,
}

pub async fn demo(demo_data: DemoData, provider: Provider<Http>) {
    info!("Voting demo ...");

    info!("Compiling merkleTree_circuit ...");

    run_command("make merkleTree_circuit").expect("Error: Failed to compile merkle tree circuit.");

    info!("MerkleTree_circuit compiled successfully.");

    let chain_id = provider.get_chainid().await.unwrap();

    let wallet = demo_data
        .pk
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(chain_id.as_u64());
    let address = wallet.address();

    info!("Voting address: {:?}", address);
    info!("Generating whitelist tree ...");

    let path = Path::new("data/whitelist.json");

    let data = fs::read_to_string(path).expect("Error: failed to read the file");

    let mut deserialize_data: Value =
        serde_json::from_str(&data).expect("Error: failed to Deserialize the data");
    let mut index = 0;
    if let Value::Array(ref mut arr) = deserialize_data {
        arr.pop();
        arr.push(json!(address));
        index = arr.len() - 1;
    }

    let serialize_data =
        serde_json::to_string_pretty(&deserialize_data).expect("Error: failed to serialize data");

    fs::write(path, serialize_data).expect("Error: failed to write the data.");

    let white_list_data = fs::read_to_string(path).expect("Error: failed to read the file");
    let mut addresses: Vec<Address> =
        serde_json::from_str(&white_list_data).expect("Error reding addresses.");

    let tree = generate_tree(&mut addresses).await;
    info!("Whitelist tree generated successfully.");

    info!("Generating whitelist proof ...");

    generate_proof(&tree, index).await;

    info!("Whitelist proof generated successfully");

    info!("Compiling vote_circuit ...");
    info!("This could take a while ...");

    run_command("make vote").expect("Error: Failed to compile vote circuit.");
    info!("Vote_circuit compiled successfully.");

    let vote_data = Vote {
        private_key: demo_data.pk,
        ceremony_id: rand::random::<u64>(),
        personal_id: rand::random::<u64>(),
        vote: 0,
        amount: PrimitiveU256::from(1),
    };

    vote(vote_data, provider).await;
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
