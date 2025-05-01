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
use structopt::StructOpt;

#[derive(Debug, StructOpt)]

pub struct DemoData {
    pk: String,
}

pub async fn demo(demo_data: DemoData) {
    info!("Voting demo ...");

    let provider: Provider<Http> = Provider::<Http>::try_from("http://localhost:8545/")
        .unwrap()
        .clone();
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

    let vote_data = Vote {
        private_key: demo_data.pk,
        ceremony_id: rand::random::<u64>(),
        personal_id: rand::random::<u64>(),
        vote: 0,
        amount: PrimitiveU256::from(1),
    };

    vote(vote_data, provider).await;
}
