use crate::commands::tally::Tally;
use crate::commands::{demo::DemoData, onchain_demo::OnchainDemoData, vote::Vote};
use alloy::primitives::U256;
use bincode::{Decode, Encode};
use chrono::{DateTime, TimeZone, Utc};
use ethers::types::U64;
use ethers::{
    providers::{self, Http, Middleware, Provider},
    types::BlockId,
};
use log::{error, info};
use std::process;
use std::sync::Arc;
use structopt::StructOpt;
use tokio::time::{timeout, Duration};

#[derive(Debug, StructOpt, Clone, Encode, Decode)]
pub struct Config {
    #[structopt(long)]
    pub network: Network,
    #[structopt(long)]
    pub ceremony_id: Option<u64>,
    #[structopt(long)]
    pub chain_id: Option<u64>,
    #[structopt(long)]
    pub votingDeadline: Option<String>,
    #[structopt(long)]
    pub tallyDeadline: Option<String>,
    #[structopt(long)]
    pub stateRoot: Option<String>,
    #[structopt(long)]
    pub result: Option<VotingResult>,
    #[structopt(long)]
    pub white_list: Vec<u64>,
    #[structopt(long)]
    pub yesVotes: Option<u64>,
    #[structopt(long)]
    pub noVotes: Option<u64>,
    #[structopt(long)]
    pub finilized: bool,
}

impl Config {
    pub async fn initiate_ceremony(&mut self) {
        let provider: Provider<Http> = Provider::<Http>::try_from(self.network.url())
            .expect("Error: failed to initiate provider.");
        if let Err(err) = check_provider(&provider).await {
            eprintln!("Provider check failed: {}", err);
            process::exit(1);
        }
        let ceremony_id = rand::random::<u64>();
        self.ceremony_id = Some(ceremony_id);
        self.chain_id = Some(provider.get_chainid().await.unwrap().as_u64());
        let white_list = vec![0; 4];
        self.white_list = white_list;
        let current_time_stamp = get_time_stamp(&provider).await;

        self.votingDeadline = Some(current_time_stamp.to_string());
        self.tallyDeadline = Some(current_time_stamp.to_string());

        info!("config: {:?}", self);
    }
    pub async fn mock_config() -> Config {
        Config {
            network: Network::Ganache,
            ceremony_id: Some(123),
            chain_id: Some(123),
            votingDeadline: Some("123".to_string()),
            tallyDeadline: Some("123".to_string()),
            stateRoot: Some("root".to_string()),
            result: Some(VotingResult::Accepted),
            white_list: [0, 0, 0, 0].to_vec(),
            yesVotes: Some(3),
            noVotes: None,
            finilized: true,
        }
    }
}

#[derive(Debug, StructOpt, Clone, Encode, Decode)]
pub enum Network {
    Ganache,
    Ethereum,
    Sepolia,
}

impl std::str::FromStr for Network {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ganache" => Ok(Network::Ganache),
            "ethereum" => Ok(Network::Ethereum),
            "sepolia" => Ok(Network::Sepolia),
            _ => Err(format!("Invalid network: {}", s)),
        }
    }
}

impl Network {
    pub fn url(&self) -> &str {
        match self {
            Network::Ganache => "http://127.0.0.1:8545",
            Network::Ethereum => "https://ethereum-rpc.publicnode.com",
            Network::Sepolia => "https://ethereum-sepolia-rpc.publicnode.com",
        }
    }
}

#[derive(Debug, StructOpt, Clone)]
pub enum Opt {
    Initiate(Config),
    Vote(Vote),
    Tally(Tally),
    Ceremonies,
    Demo(DemoData),
    OnchainDemo(OnchainDemoData),
}

// pub fn get_white_list() -> Vec<Fr> {
//     let path = Path::new("data/whitelist.json");
//     let file = File::open(path).unwrap();
//     let mut whitelist: Vec<String> = serde_json::from_reader(file).unwrap();
//     whitelist
//         .iter()
//         .map(|x| {
//             let num =
//                 PrimitiveU256::from_str_radix(x, 16).expect("Error: failed to get u256 from hash.");
//             u256_to_fp(num)
//         })
//         .collect()
// }

async fn check_provider(provider: &Provider<Http>) -> Result<(), String> {
    match timeout(Duration::from_secs(5), provider.get_block_number()).await {
        Ok(Ok(block_num)) => {
            println!("✔ Provider is up. Current block: {}", block_num);
            Ok(())
        }
        Ok(Err(rpc_err)) => Err(format!("RPC error when checking provider: {}", rpc_err)),
        Err(_) => Err("Timed out waiting for provider response".into()),
    }
}

pub async fn get_time_stamp(provider: &Provider<Http>) -> primitive_types::U256 {
    let currect_block = provider
        .get_block(ethers::types::BlockNumber::Latest)
        .await
        .expect("RPC error fetching block")
        .expect("No block data returned");
    primitive_types::U256::from(currect_block.timestamp.as_u64())
}

#[derive(Debug, StructOpt, Clone, Encode, Decode)]
pub enum VotingResult {
    Accepted,
    Rejected,
}

impl std::str::FromStr for VotingResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "accepted" => Ok(VotingResult::Accepted),
            "rejected" => Ok(VotingResult::Rejected),
            _ => Err(format!("Invalid voting result: {}", s)),
        }
    }
}
