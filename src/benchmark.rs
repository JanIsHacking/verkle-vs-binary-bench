use std::time::{Duration, Instant};
use dusk_bytes::Serializable;
use rand::seq::SliceRandom; // Add this to your dependencies in Cargo.toml
use rand::thread_rng;

use verkle_trie::{Element, TrieTrait};
use verkle_trie::proof::{ExtPresent, VerificationHint, VerkleProof};
use ipa_multipoint::multiproof::MultiPointProof;
use crate::utils::{evaluate_benchmark_results, BasicPRNG};
use rayon::prelude::*;
use crate::verkle::verkle::generate_verkle_tree;

use dusk_plonk::prelude::*;
use dusk_poseidon::{Domain, Hash};
use rand::rngs::StdRng;
use rand::{SeedableRng};
use crate::binary::tree::generate_binary_tree;
use ff::Field;
use crate::binary::circuit::OpeningCircuit;
use crate::binary::poseidon_merkle_copy::{
    Item as PoseidonItem
};

const NUMBER_OF_LEAVES_TO_PROVE: usize = 5000;
const NUM_RUNS: usize = 10usize;


pub trait Size {
    fn serialized_size(&self) -> usize;
}

impl Size for Element {
    fn serialized_size(&self) -> usize {
        self.to_bytes().len()
    }
}

impl Size for MultiPointProof {
    fn serialized_size(&self) -> usize {
        self.open_proof.to_bytes().expect("").len()
    }
}

impl Size for VerificationHint {
    fn serialized_size(&self) -> usize {
        let depths_size = size_of_val(&self.depths) + self.depths.len() * size_of::<u8>();
        let extension_present_size = size_of_val(&self.extension_present) + self.extension_present.len() * size_of::<ExtPresent>();
        let diff_stem_no_proof_size = size_of_val(&self.diff_stem_no_proof) + self.diff_stem_no_proof.len() * size_of::<[u8; 31]>();

        depths_size + extension_present_size + diff_stem_no_proof_size
    }
}

impl Size for VerkleProof {
    // Method to calculate the serialized size of the VerkleProof
    fn serialized_size(&self) -> usize {
        let hint_size = self.verification_hint.serialized_size(); // Assume this method exists
        let comms_size = self.comms_sorted.len() * 32; // Each commitment is 32 bytes
        let proof_size = self.proof.serialized_size(); // Assume this method exists for MultiPointProof

        hint_size + 4 + comms_size + proof_size // +4 for the u32 size of num_comms
    }
}

pub fn benchmark_verkle() {
    // let num_keys = vec![16_384, 65_536, 262_144, 1_048_576, 4_194_304, 16_777_216, 67_108_864, 268_435_456];
    let num_keys = vec![16_384, 65_536, 262_144, 1_048_576, 4_194_304, 16_777_216];

    // Parallelize over num_keys
    num_keys.into_par_iter().for_each(|total_keys| {
        let mut prng = BasicPRNG::default();
        let keys = prng.rand_vec_bytes(total_keys);
        let verkle_trie = generate_verkle_tree(&keys);

        println!("Starting verkle benchmark for {}", total_keys);

        let results: Vec<(Duration, Duration, usize)> = (0..NUM_RUNS).into_par_iter().map(|i| {
            println!("Run {}", i);
            // Use the first 10,000 keys for proof generation
            let proof_keys = keys.iter().take(NUMBER_OF_LEAVES_TO_PROVE).cloned().collect::<Vec<_>>();

            // Benchmark proof generation
            let start_time = Instant::now();
            let proof = verkle_trie.create_verkle_proof(proof_keys.clone().into_iter()).unwrap();
            let proof_time = start_time.elapsed();

            let size = proof.serialized_size();

            // Benchmark proof verification
            let start_verification = Instant::now();
            let values = proof_keys.clone().iter().map(|val| Some(*val)).collect();
            let (is_valid, _) = proof.check(proof_keys, values, verkle_trie.root_commitment());
            let verification_time = start_verification.elapsed();

            // Optionally, you can assert the validity here
            assert!(is_valid, "Proof verification failed");

            (proof_time, verification_time, size)
        }).collect();

        evaluate_benchmark_results(results, NUM_RUNS, total_keys, NUMBER_OF_LEAVES_TO_PROVE, "verkle".to_string());
    });

    println!("Finished benchmarking verkle.")
}

// Only one key at a time ...
// let num_keys = vec![16_384, 65_536, 262_144, 1_048_576, 4_194_304, 16_777_216, 67_108_864, 268_435_456];
// let key_logs = vec![14,     16,     18,      20,        22,        24,         26]
const NUM_KEYS: usize = 64;  // number of leaves in the tree
pub const KEY_LOG: usize = 6;  // log of the number of keys
const CAPACITY: usize = 14;  // capacity is set to 1 << 14, so 2^14. This represents the maximum number of gates in the circuit

pub fn benchmark_binary() {
    let mut rng = StdRng::seed_from_u64(0xdea1);
    println!("Starting binary benchmarking with {} keys and {} runs.", NUM_KEYS, NUM_RUNS);

    let mut keys = vec![];
    for _ in 0..NUM_KEYS {
        let hash =
            Hash::digest(Domain::Other, &[BlsScalar::random(&mut rng)])[0];
        keys.push(hash);
    }
    println!("Generating the binary tree.");
    let binary_tree = generate_binary_tree::<KEY_LOG>(keys.clone());

    println!("Starting the benchmark");
    let mut results = vec![];
    for i in 0..NUM_RUNS {
        println!("Run {}", i);
        // Use the first 10,000 keys for proof generation
        let mut rng = thread_rng();
        let proof_keys = keys
            .iter()
            .enumerate()
            .collect::<Vec<_>>()
            .choose_multiple(&mut rng, NUMBER_OF_LEAVES_TO_PROVE)
            .cloned()
            .collect::<Vec<_>>();

        // Benchmark proof generation
        let label = b"binary merkle snark";
        let pp = PublicParameters::setup(1 << CAPACITY, &mut rng).unwrap();

        let mut total_proving_time = Duration::new(0, 0);
        let mut total_verification_time = Duration::new(0, 0);
        let mut total_proof_size = 0;

        for (index, &key) in proof_keys {
            let start_proving_time = Instant::now();
            let (prover, verifier) = Compiler::compile::<OpeningCircuit>(&pp, label)
                .expect("Circuit should compile successfully");
            let opening = binary_tree.opening(index as u64).unwrap();
            let leaf = PoseidonItem::<()>::new(key, ());
            let circuit = OpeningCircuit::new(opening, leaf);

            let (proof, public_inputs) = prover
                .prove(&mut rng, &circuit)
                .expect("Proof generation should succeed");

            let proving_time = start_proving_time.elapsed();
            total_proving_time += proving_time;
            let start_verification_time = Instant::now();

            verifier
                .verify(&proof, &public_inputs)
                .expect("Proof verification should succeed");

            let verification_time = start_verification_time.elapsed();
            total_verification_time += verification_time;

            // Calculate and accumulate proof size using serde serialization
            total_proof_size += proof.to_bytes().len();
        }

        results.push((total_proving_time, total_verification_time, total_proof_size))
    }

    evaluate_benchmark_results(results, NUM_RUNS, NUM_KEYS, NUMBER_OF_LEAVES_TO_PROVE, "binary".to_string());
}
