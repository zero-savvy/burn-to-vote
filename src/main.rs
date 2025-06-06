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
            db.ceremonies().save_ceremony(cfg);
        }
        Opt::Vote(v) => {
            let mut cfg;
            if let Some(id) = v.ceremony_id {
                cfg = db.ceremonies.get_ceremony(Some(id))?;
            } else {
                cfg = db.ceremonies.get_ceremony(None)?;
            }
            commands::vote::vote(&mut cfg, v).await;
            db.ceremonies().save_ceremony(cfg)?;
        }
        Opt::Tally(t) => {
            let mut cfg;
            if let Some(id) = t.ceremony_id {
                cfg = db.ceremonies.get_ceremony(Some(id))?;
            } else {
                cfg = db.ceremonies.get_ceremony(None)?;
            }
            commands::tally::tally(&mut cfg).await;
            db.ceremonies().save_ceremony(cfg)?;
        }
        Opt::ListCeremonies =>{
            let list  =db.ceremonies().get_all_ceremonies()?;
            info!("all ceremonies: {:?}", list)
        }
        Opt::Demo(d) => {
            let mut cfg;
            if let Some(id) = d.ceremony_id {
                cfg = db.ceremonies.get_ceremony(Some(id))?;
            } else {
                cfg = db.ceremonies.get_ceremony(None)?;
            }
            commands::demo::demo(&mut cfg, d).await;
            db.ceremonies().save_ceremony(cfg)?;
        }
        Opt::OnchainDemo(d) => {
            let mut cfg;
            if let Some(id) = d.ceremony_id {
                cfg = db.ceremonies.get_ceremony(Some(id))?;
            } else {
                cfg = db.ceremonies.get_ceremony(None)?;
            }
            commands::onchain_demo::onchain_demo(&mut cfg, d).await;
            db.ceremonies().save_ceremony(cfg)?;
        }
        _ => unreachable!(),
    }
    Ok(())
}
// cargo run -- vote --amount 1 --vote 1 --revote 0 --private-key 0xfc8b5067256b891cb8d078cd657805ab62ef75888506104acf0b0001ea5c349c
