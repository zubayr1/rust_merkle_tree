use rs_merkle::MerkleTree;
use rs_merkle::algorithms::Sha256;
use sha2::{digest::FixedOutput, Digest};


fn hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = sha2::Sha256::new();

    hasher.update(data);
    <[u8; 32]>::from(hasher.finalize_fixed())
}

fn create_tree(leaf_values: Vec<String>) -> MerkleTree<Sha256>
{
    let leaves: Vec<[u8; 32]> = leaf_values
        .iter()
        .map(|x| hash(x.as_bytes()))
        .collect();

    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);

    return merkle_tree;
}

fn get_root(merkle_tree: MerkleTree<Sha256>) -> String
{
    let root = merkle_tree.root_hex().ok_or("couldn't get the merkle root").unwrap();

    return root;
}



fn main() {
    println!("Hello, world!");


    let mut leaf_values: Vec<String> = Vec::new();
    leaf_values.push("a".to_string());
    leaf_values.push("b".to_string());
    leaf_values.push("b".to_string());

    let merkle_tree = create_tree(leaf_values);

    
    let root = get_root(merkle_tree);

    println!("{}", root);
    
}
