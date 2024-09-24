use dusk_bls12_381::BlsScalar;
use crate::binary::poseidon_merkle_copy::{
    Item as PoseidonItem, Tree as PoseidonTree,
};

pub fn generate_binary_tree<const H: usize>(keys: Vec<BlsScalar>) -> PoseidonTree<(), H> {
    let mut tree = PoseidonTree::<(), H>::new();

    for (position, key) in keys.iter().enumerate() {
        let leaf = PoseidonItem::<>::new(key.clone(), ()); // Assuming `key` implements Clone
        tree.insert(position as u64, leaf);
    }
    tree
}