use anyhow::Result;
use sled::{Db, Tree};


#[derive(Debug)]
pub struct VotersT {
    voters: Tree
}

impl VotersT {
    pub fn new(db: &Db) ->Result<VotersT>{
        Ok(VotersT{
            voters: db.open_tree("voters")?
        })
    }
    
}