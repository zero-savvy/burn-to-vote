#![allow(warnings)]
mod circuits;
mod commands;
mod db;
mod utils;
use db::VotingDB;
use log::info;
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
            db.ceremonies().save_ceremony(cfg)?;
        }
        Opt::Vote(v) => {
            let mut cfg = db.ceremonies().get_ceremony(v.ceremony_id)?;
            commands::vote::vote(&mut cfg, v).await;
            db.ceremonies().save_ceremony(cfg)?;
        }
        Opt::Tally(t) => {
            let mut cfg = db.ceremonies().get_ceremony(t.ceremony_id)?;
            commands::tally::tally(&mut cfg).await;
            db.ceremonies().save_ceremony(cfg)?;
        }
        Opt::Ceremonies => {
            let list = db.ceremonies().get_all_ceremonies()?;
            info!("all ceremonies: {:?}", list)
        }
        Opt::Demo(d) => {
            let mut cfg = db.ceremonies().get_ceremony(d.ceremony_id)?;
            commands::demo::demo(&mut cfg, d).await;
            db.ceremonies().save_ceremony(cfg)?;
        }
        Opt::OnchainDemo(d) => {
            commands::onchain_demo::onchain_demo(d).await;
        }
        _ => unreachable!(),
    }
    Ok(())
}
