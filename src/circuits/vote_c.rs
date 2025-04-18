use crate::circuits::{Circuit, CircuitIdentifier};
// use primitive_types::U256;
use ethers::types::{
    serde_helpers::deserialize_stringified_numeric, Address, Bytes, H256, U256, U64,
};
use serde_json::json;
pub struct VoteCircuit {
    identifier: CircuitIdentifier<'static>,
    address: String,
    private_key: U256,
    blinding_factor: u64,
    ceremony_id: u64,
    personal_id: u64,
    vote: u64,
    nonce: U64,
    balance: U256,
    code_hash: [u8; 32],
    storage_hash: [u8; 32],
    state_root: Vec<u8>,
    account_rlp: Vec<u8>,
    account_rlp_len: usize,
    account_proof: Vec<Vec<u8>>,
    account_proof_length: usize,
    node_length: Vec<usize>,
    // node_types: Vec<usize> ,
}

impl VoteCircuit {
    pub fn new(
        address: String,
        private_key: U256,
        blinding_factor: u64,
        ceremony_id: u64,
        personal_id: u64,
        vote: u64,
        nonce: U64,
        balance: U256,
        code_hash: [u8; 32],
        storage_hash: [u8; 32],
        state_root: Vec<u8>,
        account_rlp: Vec<u8>,
        account_rlp_len: usize,
        account_proof: Vec<Vec<u8>>,
        account_proof_length: usize,
        node_length: Vec<usize>,
        // node_types: Vec<usize>
    ) -> Self {
        Self {
            identifier: CircuitIdentifier {
                circuit_name: "vote",
            },
            address,
            private_key,
            blinding_factor,
            ceremony_id,
            personal_id,
            vote,
            nonce,
            balance,
            code_hash,
            storage_hash,
            state_root,
            account_rlp,
            account_rlp_len,
            account_proof,
            account_proof_length,
            node_length,
            // node_types
        }
    }
    pub fn format_inputs(&self) -> Result<String, Box<dyn std::error::Error>> {
        let inputs = json!({
            "address": self.address,
            "privateKey":self.private_key.to_string(),
            "blinding_factor":self.blinding_factor.to_string(),
            "ceremonyID": self.ceremony_id.to_string(),
            "personalID": self.personal_id.to_string(),
            "vote": self.vote.to_string(),
            "nonce" : self.nonce,
            "balance":self.balance,
            "code_hash": self.code_hash,
            "storage_hash": self.storage_hash,
            "state_root": self.state_root,
            "account_rlp": self.account_rlp,
            "account_rlp_len": self.account_rlp_len,
            "account_proof": self.account_proof,
            "account_proof_length":self.account_proof_length,
            "node_length":self.node_length,
            // "node_types" : self.node_types
        });
        Ok(inputs.to_string())
    }
}

impl Circuit for VoteCircuit {
    fn generate_input_file(&self, inputs: String) -> Result<(), Box<dyn std::error::Error>> {
        self.identifier.generate_input_file(inputs)?;
        Ok(())
    }

    fn generate_witness(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.identifier.generate_witness()?;
        Ok(())
    }

    fn setup_zkey(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.identifier.setup_zkey()?;
        Ok(())
    }

    fn generate_proof(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.identifier.generate_proof()?;

        Ok(())
    }

    fn setup_vkey(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.identifier.setup_vkey()?;
        Ok(())
    }

    fn verify_proof(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.identifier.verify_proof()?;
        Ok(())
    }
    fn generate_verifier(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.identifier.generate_verifier()?;
        Ok(())
    }
}
