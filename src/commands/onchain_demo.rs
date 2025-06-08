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
use std::env;

#[derive(Debug, StructOpt, Clone)]
pub struct OnchainDemoData {
    pk: String,
}

abigen!(Voting, "data/abi.json");

pub async fn onchain_demo(
    demo_data: OnchainDemoData,
) -> Result<(), Box<dyn Error>> {
    info!("Voting demo ...");
    
    let public_data = get_public().await;
    
    let provider: Provider<Http> = Provider::<Http>::try_from("http://127.0.0.1:8545")?.clone();
    let current_block = provider
        .get_block(ethers::types::BlockNumber::Latest)
        .await
        .expect("RPC error fetching block")
        .expect("No block data returned");
    let current_timestamp = current_block.timestamp.as_u64();
    
    let voting_time = current_timestamp + 5;
    let tally_time = current_timestamp + 10;
    
    info!("Deploying the contracts ...");
    let deploy_command = format!(
        "cd contracts && forge script VotingScript --rpc-url http://127.0.0.1:8545 --broadcast --sig 'run(uint256,uint256,uint256,uint256,uint256)' {} {} {} {} {} && cd ..",
        voting_time,
        tally_time,
        public_data.data[2],
        public_data.data[5],
        public_data.data[0]
    );
    info!("{:?}", deploy_command);
    run_command(&deploy_command).expect("failed to deploy the contracts");
    info!("Contracts deployed...");
    
    info!("Setting up voting contract instance ...");
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

    info!("Sending the vote trx ...");
    let submit_vote_call = contract.submit_vote(proof.pi_a, proof.pi_b, proof.pi_c, public_data.data);

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

    let warp_command = format!(
        "cd contracts && cast rpc evm_increaseTime {} && cast rpc evm_mine && cd ..",
        tally_time - current_timestamp + 1
    );
    info!("Warping time forward...");
    run_command(&warp_command).expect("failed to warp time");

    info!("Calling tally votes...");
    let tally_call = contract.tally_votes();
    match tally_call.send().await {
        Ok(receipt) => {
            info!("Tally completed.");
            info!("Transaction hash: {:?}", receipt.tx_hash());
            
            let results = contract.get_results().call().await?;
            info!("Voting results - Yes votes: {}, No votes: {}", results.0, results.1);
        }
        Err(e) => {
            error!("Failed to tally votes: {:?}", e);
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
