use crate::circuits::Circuit;
use crate::{
    circuits::mpt_c::MptCircuit,
    utils::{
        account::{get_account_proof, get_account_rlp},
        get_mpt_node_type, hexToBytes, serialize_hex,
    },
};
use ethers::{
    core::types::TransactionRequest,
    middleware::SignerMiddleware,
    prelude::*,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    utils::{self, keccak256, rlp},
};
use log::info;
use serde::ser::SerializeTuple;

use hex;
use std::{clone, convert::TryFrom};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Burn {
    private_key: String,
    burn_address: Address,
    amount: U256,
}

pub async fn burn(burn_data: Burn) -> String {
    let provider = Provider::<Http>::try_from("http://localhost:8545/")
        .unwrap()
        .clone();

    let chain_id = provider.get_chainid().await.unwrap();
    let wallet: LocalWallet = burn_data
        .private_key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(chain_id.as_u64());

    let to_address = burn_data.burn_address;

    let client = SignerMiddleware::new(provider.clone(), wallet);

    let pre_tx_balance = provider
        .clone()
        .get_balance(burn_data.burn_address, None)
        .await
        .unwrap();

    let tx = TransactionRequest::new()
        .to(to_address)
        .value(U256::from(utils::parse_ether(burn_data.amount).unwrap()));

    let pending_tx = client.send_transaction(tx, None).await.unwrap();
    let receipt = pending_tx
        .await
        .unwrap()
        .ok_or_else(|| eyre::format_err!("tx dropped from mempool"))
        .unwrap();
    let tx = client
        .get_transaction(receipt.transaction_hash)
        .await
        .unwrap();

    let post_tx_balance = provider
        .clone()
        .get_balance(burn_data.burn_address, None)
        .await
        .unwrap();

    assert_eq!(
        pre_tx_balance + U256::from(utils::parse_ether(burn_data.amount).unwrap()),
        post_tx_balance
    );

    let addres_proof = get_account_proof(burn_data.burn_address).await;
    let _account_rlp = get_account_rlp(addres_proof.clone());
    let rlp_hex = hex::encode(_account_rlp.clone());

    let mut serilized_rlp = serialize_hex(&rlp_hex);
    let serilized_rlp_len = serilized_rlp.len();
    if serilized_rlp_len < 164 {
        let diff = 164 - serilized_rlp_len;
        serilized_rlp.resize(serilized_rlp_len + diff, 0);
    }

    let block = provider.get_block_number().await.unwrap();
    let bp = provider.get_block(block).await.unwrap();
    let mut block_root: [u8; 32] = [0; 32];
    match bp {
        Some(x) => {
            let root = x.state_root;
            block_root = root.as_fixed_bytes().clone();
        }
        None => println!("block number does not exist"),
    }
    let state_root_hex = hex::encode(block_root);
    let serialized_state_root = serialize_hex(&state_root_hex);

    let mut proof: Vec<Vec<u8>> = vec![];
    let mut prooflen: Vec<usize> = vec![];

    for item in &addres_proof.account_proof {
        let mut node_hex_array = serialize_hex(&hex::encode(item));
        let len = node_hex_array.len();

        prooflen.push(len);
        if len < 1064 {
            let diff = 1064 - len;
            node_hex_array.resize(len + diff, 0);
        }
        proof.push(node_hex_array);
    }

    let circuit = MptCircuit::new(
        addres_proof.nonce,
        addres_proof.balance,
        addres_proof.code_hash.to_fixed_bytes(),
        addres_proof.storage_hash.to_fixed_bytes(),
        serialized_state_root.clone(),
        serilized_rlp.clone(),
        serilized_rlp_len,
        proof.clone(),
        proof.len().clone(),
        prooflen,
    );
    info!("Burn address circuit: ");
    let inputs = circuit.format_inputs().unwrap();
    circuit.generate_input_file(inputs).unwrap();
    circuit.generate_witness().unwrap();
    circuit.setup_zkey().unwrap();
    circuit.generate_proof().unwrap();
    circuit.setup_vkey().unwrap();
    circuit.verify_proof().unwrap();
    "burn token and generate burn proof".to_string()
}
