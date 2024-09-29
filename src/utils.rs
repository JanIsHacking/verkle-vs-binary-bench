use std::fs::OpenOptions;
use csv::Writer;
use std::time::Duration;

// Code regarding BasicPRNG is from rust-verkle/verkle-trie/tests/trie_fuzzer.rs
pub struct BasicPRNG {
    seed: [u8; 32],
    counter: u64,
}

impl Default for BasicPRNG {
    fn default() -> Self {
        BasicPRNG::new([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ])
    }
}

impl BasicPRNG {
    pub fn new(seed: [u8; 32]) -> BasicPRNG {
        let counter = 0u64;
        BasicPRNG { counter, seed }
    }

    pub fn rand_bytes(&mut self) -> [u8; 32] {
        use sha2::Digest;

        let mut hasher = sha2::Sha256::new();
        hasher.update(&self.counter.to_le_bytes()[..]);
        hasher.update(&self.seed[..]);
        let res: [u8; 32] = hasher.finalize().into();

        self.counter += 1;

        res
    }

    pub fn rand_vec_bytes(&mut self, num_keys: usize) -> Vec<[u8; 32]> {
        (0..num_keys).map(|_| self.rand_bytes()).collect()
    }
}

pub fn write_benchmark_results_to_csv(
    benchmark_name: String,
    total_keys: usize,
    average_proof_time: Duration,
    average_verification_time: Duration,
    average_size: usize,
    no_runs: usize,
    leaves_to_prove: usize,
    file_path: &str,
) {
    // Open the file in read+write mode to check if it exists and is empty
    let file_exists = OpenOptions::new()
        .read(true)
        .open(file_path)
        .is_ok();

    // Now open the file in append mode to write the results
    let mut wtr = Writer::from_writer(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
            .expect("Unable to open results.csv for writing"),
    );

    // Write the header if the file was empty or newly created
    if !file_exists || file_metadata(file_path).unwrap_or(0) == 0 {
        wtr.write_record(&["name", "total_keys", "average_proof_time", "average_verification_time", "average_size", "no_runs", "leaves_to_prove"])
            .expect("Unable to write header");
    }

    // Write the benchmark results to the CSV file
    wtr.write_record(&[
        benchmark_name,
        total_keys.to_string(),
        format!("{:?}", average_proof_time),
        format!("{:?}", average_verification_time),
        average_size.to_string(),
        no_runs.to_string(),
        leaves_to_prove.to_string(),
    ])
        .expect("Unable to write benchmark result");

    // Flush to ensure data is written to disk
    wtr.flush().expect("Unable to flush CSV writer");
}

// Helper function to get file size
fn file_metadata(file_path: &str) -> std::io::Result<u64> {
    let metadata = std::fs::metadata(file_path)?;
    Ok(metadata.len())
}

pub fn evaluate_benchmark_results(results: Vec<(Duration, Duration, usize)>, num_runs: usize, total_keys: usize, leaves_to_prove: usize, name: String) {// Aggregate results
    let (total_proof_time, total_verification_time, total_size) = results.iter().fold(
        (Duration::new(0, 0), Duration::new(0, 0), 0),
        |(acc_proof, acc_verification, acc_size), &(proof_time, verification_time, size)| {
            (acc_proof + proof_time, acc_verification + verification_time, acc_size + size)
        },
    );

    let average_proof_time = total_proof_time / num_runs as u32;
    let average_verification_time = total_verification_time / num_runs as u32;
    let average_size = total_size / num_runs as usize;

    println!(
        "Average time for {} keys: {:?}, Average proof size: {} bytes, Average verification time: {:?}, Number of runs {}, Leaves to prove {}",
        total_keys, average_proof_time, average_size, average_verification_time, num_runs, leaves_to_prove
    );

    write_benchmark_results_to_csv(
        name,
        total_keys,
        average_proof_time,
        average_verification_time,
        average_size,
        num_runs,
        leaves_to_prove,
        "resources/results.csv",
    );
}
