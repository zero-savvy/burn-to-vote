use crate::utils::config::Config;
use anyhow::{anyhow, Result};
use bincode::{Decode, Encode};
use ethers::providers::{Http, Middleware, Provider};
use sled::{Db, Tree};
use structopt::StructOpt;

#[derive(Debug)]

pub struct CeremonyT {
    ceremonies: Tree,
}

impl CeremonyT {
    pub fn new(db: &Db) -> Result<Self> {
        Ok(CeremonyT {
            ceremonies: db.open_tree("ceremonies")?,
        })
    }

    pub fn save_ceremony(&self, mut cfg: Config) -> Result<()> {
        let key = cfg.ceremony_id.expect("failed to get ceremony id.");
        let value = bincode::encode_to_vec(&cfg, bincode::config::standard()).unwrap();
        self.ceremonies
            .insert(key.to_be_bytes(), value.clone())
            .expect("db insert failed");
        self.ceremonies
            .insert("currect", value)
            .expect("db insert failed");
        Ok(())
    }
    pub fn get_ceremony(&self, ceremonyId: Option<u64>) -> Result<Config> {
        match ceremonyId {
            Some(id) => {
                let key = id.to_be_bytes();
                let cfg = self.ceremonies.get(key)?;
                match cfg {
                    Some(data) => {
                        let (cfg, _) =
                            bincode::decode_from_slice(&data, bincode::config::standard())
                                .expect("Failed to decode config");
                        Ok(cfg)
                    }
                    None => Err(anyhow!("Error: no {:?} ceremony.", ceremonyId)),
                }
            }
            None => {
                let raw_data = self.ceremonies.get("currect")?;
                match raw_data {
                    Some(data) => {
                        let (cfg, _) =
                            bincode::decode_from_slice(&data, bincode::config::standard())
                                .expect("Failed to decode config");
                        Ok(cfg)
                    }
                    None => Err(anyhow!("Error: no current ceremony.")),
                }
            }
        }
    }
}
