use anyhow::Result;
use ceremony::CeremonyT;
use openssl::derive;
use sled::{Db, Tree};
use voter::VotersT;

mod ceremony;
mod voter;
mod keys;
mod values;

#[derive(Debug)]
pub struct VotingDB {
    pub db: Db,
    pub ceremonies: CeremonyT,
    pub voters: VotersT
}

impl VotingDB {

    pub fn new(path: &str)->Result<Self>{
        let db = sled::open(path)?;
        Ok(VotingDB {
            ceremonies: CeremonyT::new(&db)?,
            voters: VotersT::new(&db)?,
            db
        })
    }

    pub fn ceremonies(&self) -> &CeremonyT {
        &self.ceremonies
    }
    pub fn voters(&self) -> &VotersT {
        &self.voters
    }
    
}