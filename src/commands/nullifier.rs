use super::utils::u256_to_fp;
use primitive_types::U256;
use poseidon_rs::{Fr, FrRepr, Poseidon};
use std::fs;
use ff::PrimeField;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
pub struct Nullifier {
    private_key: String,
    ceremony_id: u64,
    blinding_factor: u64,
}

pub fn generate_nullifier(data: Nullifier) -> Fr {
    let private_key = U256::from_str_radix(&data.private_key, 16).unwrap();    
    let private_key_fp = u256_to_fp(private_key);
    
    let blinding_factor_fp = Fr::from_repr(FrRepr::from(data.blinding_factor)).unwrap();
    let ceremony_id_fp = Fr::from_repr(FrRepr::from(data.ceremony_id)).unwrap();

    let input: Vec<Fr> = vec![
        private_key_fp,
        ceremony_id_fp,
        blinding_factor_fp,
    ];

    let poseidon = Poseidon::new();
    let hash = poseidon.hash(input).unwrap();

    let hash_string = hash.into_repr().to_string();


    // Create a HashMap to store inputs
    let mut inputs = HashMap::new();
    inputs.insert("privateKey", private_key.to_string());
    inputs.insert("blindingFactor", data.blinding_factor.to_string());
    inputs.insert("ceremonyID", data.ceremony_id.to_string());
    inputs.insert("nullifier", U256::from_str_radix(&hash_string[2..], 16).unwrap().to_string());

    // Write inputs to a JSON file
    let dir_path = "circuits/nullifier";
    fs::create_dir_all(dir_path).expect("Failed to create directories");

    let file_path = format!("{}/input.json", dir_path);
    let mut file = File::create(&file_path).expect("Unable to create file");
    let json_data = serde_json::to_string_pretty(&inputs).expect("Failed to serialize JSON");
    file.write_all(json_data.as_bytes()).expect("Failed to write to file");

    hash
}