use crate::circuits::Circuit;
use crate::utils::account::prepare_mpt_data;
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
use log::{info, log};
use serde::ser::SerializeTuple;

use hex;
use std::{clone, convert::TryFrom};
use structopt::StructOpt;
type PrimitiveU256 = primitive_types::U256;
use super::super::utils::account;

#[derive(Debug, StructOpt)]
pub struct Burn {
    pub private_key: String,
    pub burn_address: Address,
    pub amount: PrimitiveU256,
}

pub async fn burn(burn_data: Burn, provider: Provider<Http>) -> (H256, Provider<Http>) {
    info!("Burnig {:?} Eth ...", burn_data.amount);

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
    info!("Burn transaction hash: {:?}", receipt.transaction_hash);

    (receipt.transaction_hash, provider)
}
