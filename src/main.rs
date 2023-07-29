use rs_merkle::MerkleTree;
use rs_merkle::MerkleProof;
use rs_merkle::algorithms::Sha256;
use sha2::{digest::FixedOutput, Digest};


extern crate reed_solomon;
use reed_solomon::Encoder;
use reed_solomon::Decoder;

use reed_solomon_erasure::galois_8::ReedSolomon;

pub fn to_shards(data: &[u8], num_nodes: usize, num_faults: usize) -> Vec<Vec<u8>> {
    let num_data_shards = num_nodes - num_faults;
    let shard_size = (data.len() + num_data_shards - 1) / num_data_shards;
    let mut data_with_suffix = data.to_vec();
    let suffix_size = shard_size * num_data_shards - data.len();
    for _ in 0..suffix_size {
        data_with_suffix.push(suffix_size as u8)
    }
    let mut result = Vec::with_capacity(num_nodes);
    for shard in 0..num_data_shards {
        result.push(data_with_suffix[shard * shard_size..(shard + 1) * shard_size].to_vec());
    }
    for _shard in 0..num_faults {
        result.push(vec![0; shard_size]);
    }
    let r = ReedSolomon::new(num_data_shards, num_faults).unwrap();
    r.encode(&mut result).unwrap();
    result
}

pub fn from_shards(mut data: Vec<Option<Vec<u8>>>, num_nodes: usize, num_faults: usize) -> Vec<u8> {
    let num_data_shards = num_nodes - num_faults;
    let r = ReedSolomon::new(num_data_shards, num_faults).unwrap();
    r.reconstruct(&mut data).unwrap();
    let mut result = Vec::with_capacity(num_data_shards * data[0].as_ref().unwrap().len());
    for shard in 0..num_data_shards {
        result.append(&mut data[shard].clone().unwrap());
    }
    result.truncate(result.len() - *result.last().unwrap() as usize);
    result
}

pub fn encoder(pvss_data: &[u8], mut e: usize) -> Vec<String>
{
    if e==0
    {
        e=1;
    }
    // Length of error correction code
    let ecc_len = 2*e;

    let enc = Encoder::new(ecc_len);
    

    // Encode data
    let encoded = enc.encode(&pvss_data[..]);

    // Simulate some transmission errors
    // let mut corrupted = *encoded;
    // for i in 0..e {
    //     corrupted[i] = 0x0;
    // }


    // let orig_str = std::str::from_utf8(pvss_data).unwrap();
    
    println!("{:?},   {:?}", encoded, encoded.ecc());

    // let dec = Decoder::new(ecc_len);

    let mut corrupted = *encoded;
    // // for i in 0..e {
    // //     corrupted[i] = 0x0;
    // // }

    // // Try to recover data
    // let known_erasures = [0];

    // let recovered = dec.correct(&mut corrupted, Some(&known_erasures)).unwrap();


    // let recv_str = std::str::from_utf8(recovered.data()).unwrap();


    println!("{:?}", corrupted);

    
    let mut leaves: Vec<String> = Vec::new();

    for i in encoded.ecc()
    {
        leaves.push(i.to_string());
    }

    return leaves;

    

}

pub fn decoder(encoded: reed_solomon::Buffer, e: usize)
{
    // Length of error correction code
    let ecc_len = 2*e;

    let dec = Decoder::new(ecc_len);
   

    // Simulate some transmission errors
    let mut corrupted = *encoded;
    // for i in 0..e {
    //     corrupted[i] = 0x0;
    // }

    // Try to recover data
    let known_erasures = [0];

    let recovered = dec.correct(&mut corrupted, Some(&known_erasures)).unwrap();


    let recv_str = std::str::from_utf8(recovered.data()).unwrap();

    println!("{:?}", recv_str);

}






fn hash(data: &[u8]) -> [u8; 32] 
{
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

fn append_to_tree(mut merkle_tree: MerkleTree<Sha256>, leaf_values: Vec<String>) -> MerkleTree<Sha256>
{
    let mut leaves: Vec<[u8; 32]> = leaf_values
        .iter()
        .map(|x| hash(x.as_bytes()))
        .collect();

    merkle_tree
    .append(&mut leaves)
    .commit();

    return  merkle_tree;
}

fn get_root(merkle_tree: MerkleTree<Sha256>) -> String
{
    let root = merkle_tree.root_hex().ok_or("couldn't get the merkle root").unwrap();

    return root;
}



fn create_proof_bytes(indices_to_prove: Vec<usize>, merkle_tree: MerkleTree<Sha256>) -> Vec<u8>
{

    let merkle_proof = merkle_tree.proof(&indices_to_prove);

    let proof_bytes = merkle_proof.to_bytes();

    return proof_bytes;

}


fn merkle_proof(proof_bytes: Vec<u8>, indices_to_prove: Vec<usize>, leaf_values_to_prove: Vec<String>, root: [u8; 32], len: usize) -> bool
{
    let proof = MerkleProof::<Sha256>::try_from(proof_bytes).unwrap();

    let leaves_to_proof: Vec<[u8; 32]>  = leaf_values_to_prove
        .iter()
        .map(|x| hash(x.as_bytes()))
        .collect();

    let leaves_to_proof = leaves_to_proof.get(0..1).ok_or("can't get leaves to prove").unwrap();

    println!("{:?}", leaves_to_proof);


    if proof.verify(root, &indices_to_prove, leaves_to_proof, len)
    {
        return true;
    }

    return false;
}


fn main() {

    let original_data = "Hello".as_bytes();
    let num_nodes = 16;      // Total number of shards
    let num_faults = 7;      // Maximum number of erasures to tolerate

    // Encoding
    let shards = to_shards(original_data, num_nodes, num_faults);

    println!("SHARDS: {:?}", shards);

    let mut received: Vec<_> = shards.iter().cloned().map(Some).collect();
    received[0] = None;
    received[2] = None;
    received[4] = None;
    received[6] = None;

    received[8] = None;
    received[10] = None;
    received[12] = None;

    let reconstructed = from_shards(received, num_nodes, num_faults);

    println!("RECONSTRUCTED: {:?}", reconstructed);

    let string: String = String::from_utf8_lossy(&reconstructed).into();

    println!("String: {}", string);


    // let leaves = encoder(b"pvss_data",  1);

    // println!("{:?}", leaves);

    // let e = 2;

    // let leaves = encoder(b"pvss_data",  e);

    // println!("{:?}", leaves);

    // let ecc_len = 2*e;

    // let enc = Encoder::new(ecc_len);

    // let converted_data: Vec<u8> = leaves.iter()
    //                 .map(|s| s.parse::<u8>().expect("Failed to convert to u8"))
    //                 .collect();




    // let encoded = enc.encode(&converted_data[..]);

    // println!("{:?}", encoded);

    // decoder(buffer, ecc_len/2);






    // let index = 2;

    // let mut leaf_values: Vec<String> = Vec::new();
    // leaf_values.push("a".to_string());
    // leaf_values.push("b".to_string());
    // leaf_values.push("c".to_string());

    // let merkle_tree = create_tree(leaf_values.clone());

    
    // let root = get_root(merkle_tree.clone());

    // println!("{}", root);


    // let mut leaf_values: Vec<String> = Vec::new();
    // leaf_values.push("d".to_string());
    // leaf_values.push("e".to_string());


    // let merkle_tree: MerkleTree<Sha256> = append_to_tree(merkle_tree.clone(), leaf_values.clone());

    // let root = get_root(merkle_tree.clone());

    // println!("{:?}", hash(root.as_bytes()));

  
    // let mut leaf_values_to_prove: Vec<String> = Vec::new(); 
    // leaf_values_to_prove.push("x".to_string());


    
    // let indices_to_prove = vec![index];

    // let proof_bytes = create_proof_bytes(indices_to_prove.clone(), merkle_tree.clone());

    // let merkle_root = merkle_tree.root().ok_or("couldn't get the merkle root").unwrap();


    // let proof = merkle_proof(proof_bytes, indices_to_prove, leaf_values_to_prove, merkle_root, merkle_tree.leaves_len());

    // println!("{}", proof);


   

}
