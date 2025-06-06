use ethers::{
    core::types::TransactionRequest,
    middleware::SignerMiddleware,
    prelude::*,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    utils::parse_ether,
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
    info!("Burnig {:?} Eth ...", burn_data.amount);

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
    let raw_wei = parse_ether("0.0001").unwrap(); // U256
    let scaled = raw_wei / U256::exp10(15);

    let balance: U256 = provider.get_balance(wallet.address(), None).await.unwrap();
    info!("balance{:?}", balance);

    let tx = TransactionRequest::new()
        .to(to_address)
        .value(U256::from(150));

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
        pre_tx_balance + U256::from(parse_ether(burn_data.amount).unwrap()),
        post_tx_balance
    );
    info!("Burn transaction hash: {:?}", receipt.transaction_hash);

    (receipt.transaction_hash, provider)
}
