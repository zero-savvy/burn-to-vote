use ethers::{
    core::types::TransactionRequest,
    middleware::SignerMiddleware,
    prelude::*,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    utils,
};
use eyre::Result;
use structopt::StructOpt;

use std::convert::TryFrom;

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

    let chain_id = provider.clone().get_chainid().await.unwrap();
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

    println!("Sent tx: {}\n", serde_json::to_string(&tx).unwrap());
    println!("Tx receipt: {}", serde_json::to_string(&receipt).unwrap());
    assert_eq!(pre_tx_balance + U256::from(utils::parse_ether(burn_data.amount).unwrap()), post_tx_balance );

    "burn token and generate burn proof".to_string()
    // to do : add generate proof
}
