#![allow(warnings)]
mod circuits;
mod commands;
mod db;
mod utils;
use db::VotingDB;
use std::{path::Path, sync::Arc};
use structopt::StructOpt;
use utils::config::Opt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .init();

    let db = VotingDB::new("voting_db")?;
    let opt = Opt::from_args();

    match opt {
        Opt::Initiate(mut cfg) => {
            cfg.initiate_ceremony().await;
            db.ceremonies().save_ceremony(cfg);
        }
        other => {
            let mut cfg = db.ceremonies.get_ceremony(None)?;

            let res = match other {
                Opt::Vote(v) => commands::vote::vote(&mut cfg, v).await,
                Opt::Tally(id) => commands::tally::tally(&mut cfg, id).await,
                Opt::Demo(d) => commands::demo::demo(&mut cfg, d).await,
                Opt::OnchainDemo(d) => commands::onchain_demo::onchain_demo(&mut cfg, d).await,
                _ => unreachable!(),
            };

            match res {
                Ok(()) => {
                    db.ceremonies().save_ceremony(cfg)?;
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                }
            }
        }
    }
    Ok(())
}
