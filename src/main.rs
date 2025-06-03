#![allow(warnings)]
mod circuits;
mod commands;
mod utils;
mod db;
use std::{path::Path, sync::Arc};
use anyhow::Ok;
use db::VotingDB;
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
            let cfg = db.ceremonies.get_ceremony(None)?;

            let res = match other {
                Opt::Vote(v) => commands::vote::vote(cfg, v).await,
                Opt::Demo(d) => commands::demo::demo(cfg, d).await,
                Opt::OnchainDemo(d) => commands::onchain_demo::onchain_demo(cfg, d).await,
                _ => unreachable!(),
            };

            if let Err(e) = res {
                eprintln!("Error: {:?}", e);
            }
        }
    }
    Ok(())
}
