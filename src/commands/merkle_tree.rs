use crate::circuits::merkle_tree_c::*;
use crate::circuits::Circuit;
use crate::utils::mt::Proof;
use crate::utils::{hash_address, mt::MerkleTree};
use alloy::primitives::{address, Address};
use ff::PrimeField;
use poseidon_rs::{Fr, FrRepr, Poseidon};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use structopt::StructOpt;

pub async fn generate_tree() -> MerkleTree {
    let path = Path::new("data/whitelist.json");
    let file = File::open(path).unwrap();
    let whitelist: Vec<Address> = serde_json::from_reader(file).unwrap();
    let depth: u32 = (whitelist.len()).ilog2();
    let mut tree = MerkleTree::new(depth as usize, whitelist);

    tree.build_tree();
    tree
}

#[derive(StructOpt, Debug)]
pub struct UserIndex {
    pub index: usize,
}

pub async fn generate_proof(tree: MerkleTree, index: usize) -> Proof {
    let proof = tree.generate_proof(index);
    let circuit = MtCircuit::new(proof.clone());
    let inputs = circuit.format_inputs().unwrap();
    circuit.generate_input_file(inputs).unwrap();
    circuit.generate_witness().unwrap();
    circuit.setup_zkey().unwrap();
    circuit.generate_proof().unwrap();
    circuit.setup_vkey().unwrap();
    circuit.verify_proof().unwrap();
    circuit.generate_verifier().unwrap();

    proof
}

#[cfg(test)]
mod tests {

    use super::*;
    #[tokio::test]
    async fn test() {
        let addresses = vec![
            Address::from([1; 20]),
            Address::from([2; 20]),
            Address::from([3; 20]),
            Address::from([4; 20]),
        ];
        let addr: Vec<Fr> = addresses.iter().map(|addr| hash_address(*addr)).collect();
        let tree = generate_tree().await;
        let proof = generate_proof(tree, 2).await;
        let root = proof.root;
        let hasher = Poseidon::new();
        let node0 = hasher.hash([addr[0], addr[1]].to_vec()).unwrap();
        let node1 = hasher.hash([addr[2], addr[3]].to_vec()).unwrap();
        let expected_root = hasher.hash([node0, node1].to_vec()).unwrap();
        assert_eq!(root, expected_root);
    }
}
