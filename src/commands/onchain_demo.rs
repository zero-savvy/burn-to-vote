use super::super::utils::proof::{decode_revert, get_contract_address, get_proof, get_public};
use crate::utils::config::Config;
use crate::utils::run_command;
use ethers::prelude::*;
use ethers::providers::{Http, Middleware, Provider};
use log::error;
use log::info;
use std::error::Error;
use std::sync::Arc;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
pub struct OnchainDemoData {
    pub ceremony_id: Option<u64>,
    pk: String,
}

abigen!(Voting, "data/abi.json");

pub async fn onchain_demo(
    config: &mut Config,
    demo_data: OnchainDemoData,
) -> Result<(), Box<dyn Error>> {
    info!("Voting demo ...");
    info!("Deploying the contracts ...");
    let deploy_command = format!(
        " cd contracts && forge script VotingScript --rpc-url {} --broadcast && cd ..",
        config.network.url()
    );
    run_command(&deploy_command).expect("failed to deploy the contracts");
    info!("Contracts deployed...");
    info!("Setting up voting contract instance ...");
    let provider: Provider<Http> = Provider::<Http>::try_from(config.network.url())?.clone();
    let chain_id = provider.get_chainid().await.unwrap();

    let wallet = demo_data
        .pk
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(chain_id.as_u64());

    let client = Arc::new(SignerMiddleware::new(provider, wallet));
    let addr = get_contract_address();
    let address = addr.parse::<Address>()?;
    let contract = Voting::new(address, client.clone());
    info!("contract address: {:?}", address);

    info!("Loading the proof data ...");

    let proof = get_proof().await;
    let public = get_public().await;

    info!("Sending the vote trx ...");

    let submit_vote_call = contract.submit_vote(proof.pi_a, proof.pi_b, proof.pi_c, public.data);

    match submit_vote_call.send().await {
        Ok(receipt) => {
            info!("Vote submitted.");
            info!("Transaction hash: {:?}", receipt.tx_hash());
        }
        Err(e) => {
            error!("Failed to submit proof: {:?}", e);
            if let Some(revert_data) = e.as_revert() {
                match decode_revert(&revert_data) {
                    Some(msg) => error!("Smart Contract Error: {}", msg),
                    None => error!("Failed to decode revert message"),
                }
            }

            return Err(e.into());
        }
    }

    Ok(())
}
