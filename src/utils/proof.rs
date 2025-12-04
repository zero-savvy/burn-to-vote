type Eth256 = ethers::types::U256;
use ethabi::{ParamType, Token};
use ethers::types::Bytes;
use log::error;
use serde::Deserialize;
use serde_json::Value;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct ProofData {
    pub pi_a: [Eth256; 2],
    pub pi_b: [[Eth256; 2]; 2],
    pub pi_c: [Eth256; 2],
}

#[derive(Debug, Deserialize)]
pub struct PublicData {
    pub data: [Eth256; 6],
}

pub async fn get_proof() -> ProofData {
    let proof_file =
        fs::read_to_string("data/vote_proof.json").expect("failed to load proof file.");
    let raw_value: serde_json::Value =
        serde_json::from_str(&proof_file).expect("failed to parse the proof data.");
    let proof_a: Vec<Eth256> = parse_u256_vec(raw_value["pi_a"].as_array().unwrap());
    let proof_c: Vec<Eth256> = parse_u256_vec(raw_value["pi_c"].as_array().unwrap());
    let proof_b: Vec<Vec<Eth256>> = raw_value["pi_b"]
        .as_array()
        .unwrap()
        .iter()
        .map(|inner| parse_u256_vec(inner.as_array().unwrap()))
        .collect();
    let pi_a: [Eth256; 2] = [proof_a[0], proof_a[1]];
    let pi_b: [[Eth256; 2]; 2] = [
        [proof_b[0][1], proof_b[0][0]],
        [proof_b[1][1], proof_b[1][0]],
    ];
    let pi_c: [Eth256; 2] = [proof_c[0], proof_c[1]];

    ProofData { pi_a, pi_b, pi_c }
}

pub async fn get_public() -> PublicData {
    let public_file =
        fs::read_to_string("data/vote_public.json").expect("failed to load public data file.");
    let raw_value: serde_json::Value =
        serde_json::from_str(&public_file).expect("failed to parse public data.");
    let data: Vec<Eth256> = parse_u256_vec(raw_value.as_array().unwrap());
    let data: [Eth256; 6] = data.try_into().expect("Expected a Vec of length 6");

    PublicData { data }
}

fn parse_u256_vec(arr: &Vec<Value>) -> Vec<ethers::types::U256> {
    arr.iter()
        .map(|v| {
            ethers::types::U256::from_dec_str(v.as_str().expect("Not a string"))
                .expect("Invalid U256 string")
        })
        .collect()
}

pub fn get_contract_address() -> String {
    let path = "contracts/broadcast/Voting.s.sol/1337/run-latest.json";
    let file = fs::read_to_string(path).expect("failed to parse trx file.");
    let raw_data: serde_json::Value = serde_json::from_str(&file).expect("failed to get trx data.");
    let mut addr = String::from("");

    if let Some(data) = raw_data["transactions"].as_array() {
        for tx in data {
            if let Some(function) = tx["function"].as_str() {
                if function == "deployVotingContract(bytes32,uint8,address,uint256,uint256,uint256,uint256,uint256)" {
                    if let Some(additional_contracts) = tx["additionalContracts"].as_array() {
                        if let Some(contract) = additional_contracts.first() {
                            addr = contract["address"]
                                .as_str()
                                .expect("failed to parse contract address")
                                .to_string();
                        }
                    }
                }
            }
        }
    }

    if addr == String::from("") {
        error!("The contract address was not found.")
    }
    addr
}


pub fn get_token_contract_address() ->String{
    let path = "contracts/broadcast/AuctionErc20.s.sol/1337/run-latest.json";
    let file = fs::read_to_string(path).expect("failed to parse trx file.");
    let raw_data: serde_json::Value = serde_json::from_str(&file).expect("failed to get trx data.");
    let mut addr = String::from("");
    if let Some(data) = raw_data["transactions"].as_array() {
        for tx in data {
                    if let Some(address) = tx.get("contractAddress") {
                            addr = address
                                .as_str()
                                .expect("failed to parse token contract address")
                                .to_string();
                    }
        }
    }

    if addr == String::from("") {
        error!("The contract address was not found.")
    }
    addr

}

pub fn decode_revert(data: &Bytes) -> Option<String> {
    if data.len() < 4 || data[0..4] != [0x08, 0xc3, 0x79, 0xa0] {
        return None;
    }

    ethabi::decode(&[ParamType::String], &data[4..])
        .ok()
        .and_then(|decoded| match decoded.first() {
            Some(Token::String(s)) => Some(s.clone()),
            _ => None,
        })
}
