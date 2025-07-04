use super::vote::{vote, Vote};
type PrimitiveU256 = primitive_types::U256;
use crate::utils::config::Config;
use crate::utils::run_command;
use log::info;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]

pub struct DemoData {
    pub ceremony_id: Option<u64>,
    pk: String,
}

pub async fn demo(
    config: &mut Config,
    demo_data: DemoData,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Voting demo ...");

    run_command("make vote").expect("Error: Failed to compile vote circuit.");
    info!("Vote_circuit compiled successfully.");

    let vote_data = Vote {
        ceremony_id: demo_data.ceremony_id,
        private_key: demo_data.pk,
        vote: 1,
        revote: 0,
        amount: PrimitiveU256::from(1),
    };

    vote(config, vote_data).await
}
