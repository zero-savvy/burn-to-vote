use log::info;
use structopt::StructOpt;
use crate::commands::{demo::DemoData, onchain_demo::OnchainDemoData, vote::Vote};
use bincode::{Decode, Encode};
use ethers::providers::{Http, Middleware, Provider};
use std::sync::Arc;

#[derive(Debug, StructOpt, Clone, Encode, Decode)]
pub struct Config {
    pub network: Network,
    pub ceremony_id: Option<u64>,
    pub chain_id: Option<u64>,
    pub white_list: Vec<u64>,
}

impl Config {
    pub async fn initiate_ceremony(&mut self) {
        let provider: Provider<Http> = Provider::<Http>::try_from(self.network.url())
            .expect("Error: failed to initiate provider.");
        let ceremony_id = rand::random::<u64>();
        self.ceremony_id = Some(ceremony_id);
        self.chain_id = Some(provider.get_chainid().await.unwrap().as_u64());
        let white_list = vec![0; 4];
        self.white_list = white_list;
        info!("config: {:?}",self);
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

