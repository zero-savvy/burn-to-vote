use crate::circuits::{Circuit, CircuitIdentifier};
// use primitive_types::U256;
use ethers::types::{serde_helpers::deserialize_stringified_numeric, Address, Bytes, H256, U64};
type EthersU256 = ethers::types::U256;
use serde_json::json;
pub struct MptCircuit {
    identifier: CircuitIdentifier<'static>,
    pub address: Vec<u8>,
    pub nonce: U64,
    pub balance: EthersU256,
    pub code_hash: [u8; 32],
    pub storage_hash: [u8; 32],
    pub state_root: Vec<u8>,
    pub account_rlp: Vec<u8>,
    pub account_rlp_len: usize,
    pub account_proof: Vec<Vec<u8>>,
    pub account_proof_length: usize,
    pub node_length: Vec<usize>,
    pub leaf_nibbles: usize,
}

impl MptCircuit {
    pub fn new(
        address: Vec<u8>,
        nonce: U64,
        balance: EthersU256,
        code_hash: [u8; 32],
        storage_hash: [u8; 32],
        state_root: Vec<u8>,
        account_rlp: Vec<u8>,
        account_rlp_len: usize,
        account_proof: Vec<Vec<u8>>,
        account_proof_length: usize,
        node_length: Vec<usize>,
        leaf_nibbles: usize,
    ) -> Self {
        Self {
            identifier: CircuitIdentifier {
                circuit_name: "mpt",
            },
            address,
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
            leaf_nibbles,
        }
    }
    pub fn format_inputs(&self) -> Result<String, Box<dyn std::error::Error>> {
        let inputs = json!({
            "address": self.address,
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
            "leaf_nibbles" : self.leaf_nibbles
        });
        Ok(inputs.to_string())
    }
}

impl Circuit for MptCircuit {
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
