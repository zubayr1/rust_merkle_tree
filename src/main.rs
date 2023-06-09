use rs_merkle::MerkleTree;
use rs_merkle::algorithms::Sha256;
use sha2::{digest::FixedOutput, Digest};


fn hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = sha2::Sha256::new();

    hasher.update(data);
    <[u8; 32]>::from(hasher.finalize_fixed())
}


fn main() {
    println!("Hello, world!");

    let leaf_values = ["a", "b", "c"];

    let leaves: Vec<[u8; 32]> = leaf_values
        .iter()
        .map(|x| hash(x.as_bytes()))
        .collect();
    

    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);

    let root = merkle_tree.root_hex().ok_or("couldn't get the merkle root").unwrap();

    println!("{}", root);
    assert_eq!(
        root,
        "7075152d03a5cd92104887b476862778ec0c87be5c2fa1c0a90f87c49fad6eff".to_string()
    );
}
