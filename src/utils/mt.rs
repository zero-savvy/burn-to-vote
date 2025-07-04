use poseidon_rs::{Fr, Poseidon};

#[derive(Debug, Clone)]
struct MerkleTreeNode {
    left: Fr,
    right: Fr,
    value: Fr,
}

impl MerkleTreeNode {
    fn new(left: Fr, right: Fr) -> Self {
        let list = vec![left, right];
        let hasher = Poseidon::new();
        let value = hasher.hash(list).unwrap();
        MerkleTreeNode { left, right, value }
    }
}

#[derive(Debug, Clone)]
pub struct Proof {
    pub root: Fr,
    pub leaf: Fr,
    pub pathElements: Vec<Fr>,
    pub pathIndices: Vec<usize>,
}

#[derive(Debug)]

pub struct MerkleTree<'a> {
    height: usize,
    leaves: &'a mut Vec<Fr>,
    root: Option<MerkleTreeNode>,
    inner_nodes: Vec<Vec<MerkleTreeNode>>,
}

impl<'a> MerkleTree<'a> {
    pub fn new(leaves: &'a mut Vec<Fr>) -> Self {
        assert!(
            is_power_of_two(leaves.len()),
            "Error: the whitelist lenghth is incorrect."
        );
        let height = ((leaves.len()) as f64).log2() as usize;
        MerkleTree {
            height,
            leaves,
            root: None,
            inner_nodes: Vec::new(),
        }
    }

    pub fn build_tree(&mut self) {
        let fr_leaves: Vec<Fr> = self.leaves.to_vec();

        self.inner_nodes = Vec::with_capacity(self.height - 1);
        for _ in 0..(self.height - 1) {
            self.inner_nodes.push(Vec::new());
        }

        for j in (0..fr_leaves.len()).step_by(2) {
            let node = MerkleTreeNode::new(fr_leaves[j].clone(), fr_leaves[j + 1].clone());
            self.inner_nodes[0].push(node);
        }

        for i in 1..self.inner_nodes.len() {
            let length = self.inner_nodes[i - 1].len();
            for j in (0..length).step_by(2) {
                let node = MerkleTreeNode::new(
                    self.inner_nodes[i - 1][j].value.clone(),
                    self.inner_nodes[i - 1][j + 1].value.clone(),
                );
                self.inner_nodes[i].push(node);
            }
        }

        let last = self.inner_nodes.last().unwrap();
        self.root = Some(MerkleTreeNode::new(
            last[0].value.clone(),
            last[1].value.clone(),
        ));
    }
    pub fn generate_proof(&self, leaf_index: usize) -> Proof {
        if self.root.is_none() {
            panic!("Merkle tree has not been built.");
        }
        if leaf_index >= self.leaves.len() {
            panic!("Leaf index {} out of bounds.", leaf_index);
        }

        let leaf_fr = self.leaves[leaf_index];

        let mut path_elements: Vec<Fr> = Vec::new();
        let mut path_indices: Vec<usize> = Vec::new();

        let sibling_index = if leaf_index % 2 == 0 {
            leaf_index + 1
        } else {
            leaf_index - 1
        };
        let sibling_fr = self.leaves[sibling_index];
        path_elements.push(sibling_fr);
        path_indices.push(leaf_index % 2);

        let mut parent_index = leaf_index / 2;
        for level in 0..self.inner_nodes.len() {
            let level_nodes = &self.inner_nodes[level];
            let sibling_parent_index = if parent_index % 2 == 0 {
                parent_index + 1
            } else {
                parent_index - 1
            };

            let sibling_hash = level_nodes[sibling_parent_index].value.clone();
            path_elements.push(sibling_hash);
            path_indices.push(parent_index % 2);
            parent_index /= 2;
        }

        let root_fr = self.root.as_ref().unwrap().value.clone();

        Proof {
            root: root_fr,
            leaf: leaf_fr,
            pathElements: path_elements,
            pathIndices: path_indices,
        }
    }
}

fn is_power_of_two(n: usize) -> bool {
    n != 0 && n & (n - 1) == 0
}
