pub use sha256::digest;
// merkle tree should contain 1024 elements 
const CHILDREN_LENGTH: usize = 1024;
const DATA_ITEM_PREFIX: &str = "data item ";

#[derive(Debug)]
pub struct MerkleProof {
    pub root: String,
    pub proof_hashes: Vec<String>
}

impl MerkleProof {
    pub fn generate_tree_components(initial_position: usize) -> MerkleProof {
        unimplemented!("write your code here")
    }
}