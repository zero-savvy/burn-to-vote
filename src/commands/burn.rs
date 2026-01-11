use ethers::{
    core::types::{TransactionRequest, U256 as EthersU256},
    middleware::SignerMiddleware,
    prelude::*,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    utils::{format_ether, parse_ether},
};
use log::info;
use structopt::StructOpt;
type PrimitiveU256 = primitive_types::U256;

#[derive(Debug, StructOpt, Clone)]
pub struct Burn {
    pub private_key: String,
    pub burn_address: Address,
    pub amount: PrimitiveU256,
}

pub async fn burn(
    provider: Provider<Http>,
    burn_address: Address,
    amount: PrimitiveU256,
    private_key: String,
) -> (H256, Provider<Http>) {
    let burn_data = Burn {
        private_key,
        burn_address,
        amount,
    };
    info!("Burnig {:?} ETH ...", burn_data.amount);

    let chain_id = provider.get_chainid().await.unwrap();
    let wallet: LocalWallet = burn_data
        .private_key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(chain_id.as_u64());

    let to_address = burn_data.burn_address;

    let client = SignerMiddleware::new(provider.clone(), wallet.clone());

    let pre_tx_balance = provider
        .clone()
        .get_balance(burn_data.burn_address, None)
        .await
        .unwrap();

    let balance: U256 = provider.get_balance(wallet.address(), None).await.unwrap();
    info!("balance: {:?} ETH", format_ether(balance));

    let tx = TransactionRequest::new()
        .to(to_address)
        .value(EthersU256::from(burn_data.amount.as_u128()));

    let pending_tx = client.send_transaction(tx, None).await.unwrap();
    let receipt = pending_tx
        .await
        .unwrap()
        .ok_or_else(|| eyre::format_err!("tx dropped from mempool"))
        .unwrap();
    let _tx = client
        .get_transaction(receipt.transaction_hash)
        .await
        .unwrap();

    let post_tx_balance = provider
        .clone()
        .get_balance(burn_data.burn_address, None)
        .await
        .unwrap();

    assert_eq!(
        pre_tx_balance + EthersU256::from(burn_data.amount.as_u128()),
        post_tx_balance,
        "Balance mismatch: expected {} wei increase, got {} wei",
        burn_data.amount,
        post_tx_balance - pre_tx_balance
    );

    info!("Burn transaction hash: {:?}", receipt.transaction_hash);

    info!("Balance difference: {} wei", post_tx_balance - pre_tx_balance);

    (receipt.transaction_hash, provider)
}