use crate::utils::config::{get_time_stamp, Config};
use log::info;
use structopt::StructOpt;
use ethers::{
    providers::{self, Http, Middleware, Provider},
    types::BlockId,
};
use primitive_types::U256;
#[derive(Debug, StructOpt, Clone)]
pub struct Tally {
    ceremoni_id: u64,
}
pub async fn tally(
    config: &mut Config,
    ceremony_id: Tally,
) -> Result<(), Box<dyn std::error::Error>> {
    let provider: Provider<Http> = Provider::<Http>::try_from(config.clone().network.url())
    .expect("Error: failed to initiate provider.");
    let current_ts:primitive_types::U256 = get_time_stamp(&provider).await;
    let tally_deadline = config
    .clone()
    .tallyDeadline
    .unwrap();
    let voting_ts = U256::from_dec_str(&tally_deadline)?;

    if current_ts > voting_ts.into() {
        let yesVotes = config.clone().yesVotes;
        let noVotes = config.clone().noVotes;
        info!("Yes vote counts: {:?}", yesVotes);
        info!("No vote counts: {:?}", noVotes);
        config.finilized = true;

        if yesVotes > noVotes {
            config.result = Some(true)
        }else {
            config.result = Some(false)
        }

        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Voting period is not over.",
        )))
    }
}
