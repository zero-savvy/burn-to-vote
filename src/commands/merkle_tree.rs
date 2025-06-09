use crate::utils::mt::MerkleTree;
use crate::utils::mt::Proof;
use poseidon_rs::Fr;
use structopt::StructOpt;

pub async fn generate_tree<'a>(whitelist: &'a mut Vec<Fr>) -> MerkleTree<'a> {
    let mut tree = MerkleTree::new(whitelist);

    tree.build_tree();
    tree
}

#[derive(StructOpt, Debug, Clone, Copy)]
pub struct UserIndex {
    pub index: usize,
}

pub async fn generate_proof<'a>(tree: &'a MerkleTree<'a>, index: usize) -> Proof {
    let proof = tree.generate_proof(index);
    // let circuit = MtCircuit::new(proof.clone());
    // let inputs = circuit.format_inputs().unwrap();
    // circuit.generate_input_file(inputs).unwrap();
    // circuit.generate_witness().unwrap();
    // circuit.setup_zkey().unwrap();
    // circuit.generate_proof().unwrap();
    // circuit.setup_vkey().unwrap();
    // circuit.verify_proof().unwrap();
    // circuit.generate_verifier().unwrap();

    proof
}

#[cfg(test)]
mod tests {
    use ff::PrimeField;
    use poseidon_rs::{Fr, Poseidon};

    use super::*;
    #[tokio::test]
    async fn test() {
        let mut addr: Vec<Fr> = [
            Fr::from_str("0").unwrap(),
            Fr::from_str("1").unwrap(),
            Fr::from_str("2").unwrap(),
            Fr::from_str("3").unwrap(),
        ]
        .to_vec();
        let tree = generate_tree(&mut addr).await;
        let proof = generate_proof(&tree, 2).await;
        let root = proof.root;
        let hasher = Poseidon::new();
        let node0 = hasher.hash([addr[0], addr[1]].to_vec()).unwrap();
        let node1 = hasher.hash([addr[2], addr[3]].to_vec()).unwrap();
        let expected_root = hasher.hash([node0, node1].to_vec()).unwrap();
        assert_eq!(root, expected_root);
    }
}
