use crate::circuits::{Circuit, CircuitIdentifier};
type PrimitiveU256 = primitive_types::U256;
use ethers::types::{U256, U64};
use ff::PrimeField;
use poseidon_rs::Fr;
use serde_json::json;

pub struct VoteCircuit {
    identifier: CircuitIdentifier<'static>,
    address: String,
    secret: PrimitiveU256,
    blinding_factor: u64,
    ceremony_id: u64,
    random_secret: u64,
    vote: u64,
    revote: u64,
    nullifier: String,
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
    leaf_nibbles: usize,
    mt_root: Fr,
    mt_leaf: Fr,
    mt_pathElements: Vec<Fr>,
    mt_pathIndices: Vec<usize>,
}

impl VoteCircuit {
    pub fn new(
        address: String,
        secret: PrimitiveU256,
        blinding_factor: u64,
        ceremony_id: u64,
        random_secret: u64,
        vote: u64,
        revote: u64,
        nullifier: String,
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
        leaf_nibbles: usize,
        mt_root: Fr,
        mt_leaf: Fr,
        mt_pathElements: Vec<Fr>,
        mt_pathIndices: Vec<usize>,
    ) -> Self {
        Self {
            identifier: CircuitIdentifier {
                circuit_name: "vote",
            },
            address,
            secret,
            blinding_factor,
            ceremony_id,
            random_secret,
            vote,
            revote,
            nullifier,
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
            mt_root,
            mt_leaf,
            mt_pathElements,
            mt_pathIndices,
        }
    }
    pub fn format_inputs(&self) -> Result<String, Box<dyn std::error::Error>> {
        let path_elements: Vec<String> = self
            .mt_pathElements
            .iter()
            .map(|p| p.into_repr().to_string())
            .collect();
        let inputs = json!({
            "address": self.address,
            "secret":self.secret.to_string(),
            "blinding_factor":self.blinding_factor.to_string(),
            "ceremonyID": self.ceremony_id.to_string(),
            "random_secret": self.random_secret.to_string(),
            "vote": self.vote.to_string(),
            "revote":self.revote,
            "nullifier": self.nullifier,
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
            "leaf_nibbles" : self.leaf_nibbles,
            "mt_root": self.mt_root.into_repr().to_string(),
            "mt_leaf": self.mt_leaf.into_repr().to_string(),
            "mt_pathElements": path_elements,
            "mt_pathIndices": self.mt_pathIndices
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
        self.identifier.generate_witness()
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
        self.identifier.verify_proof()
    }
    fn generate_verifier(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.identifier.generate_verifier()
    }
}
