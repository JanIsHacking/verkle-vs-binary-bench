# Verkle vs Binary Merkle Tree Benchmarking

This project benchmarks the performance of Verkle trees and binary Merkle trees for Ethereum state representation. It evaluates three key metrics: proving time, proof size, and verification time, across various tree sizes and different hardware configurations.

## Methodology

The benchmarks are conducted on two different machines to account for hardware variability. For both Verkle and binary Merkle trees, tree sizes ranging from $2^5$ to $2^{24}$ nodes are tested. Each test is averaged over ten runs to minimize variability in runtime.

### Machines Used:
| \#  | Operating System   | CPU                    | RAM      |
| --- | ------------------ | ---------------------- | -------- |
| 1   | Windows 10 22H2     | Intel i5-4690K         | 22 GiB   |
| 2   | Ubuntu LTS 22.04    | AMD Ryzen 5975WX 32-Core | 125 GiB  |

## Project Structure

- **Verkle Trees**: Implementation based on the `rust-verkle` library, modified to include proof size measurements and integrated into the benchmarking framework.
- **Binary Merkle Trees**: Uses the `poseidon-merkle` library, adapted to use a tree arity of 2. SNARK proofs are generated and verified using the `dusk-plonk` library.

## Benchmarking Metrics

The benchmarks focus on the following key metrics:

1. **Proving Time**: Time taken to generate a proof for a specific number of leaves.
2. **Proof Size**: Size of the generated proof in bytes.
3. **Verification Time**: Time taken to verify the generated proof.

## Prerequisites

- **Rust**: Ensure you have Rust installed on your machine.
- **rayon**: For parallelization support.
- **dusk-plonk**: For generating and verifying SNARK proofs.
- **poseidon-merkle**: For binary Merkle tree implementations.
- **rust-verkle**: For Verkle tree implementations.

### Installation

1. Clone this repository:
   ```sh
   git clone https://github.com/yourusername/verkle_vs_binary_bench.git
   cd verkle_vs_binary_bench
   ```

2. Install dependencies:
   ```sh
   cargo build
   ```

## Running the Benchmarks

To run the benchmarks for both Verkle and binary Merkle trees:

```sh
cargo run --release
```

This will generate the results for each tree size, including the proving time, proof size, and verification time for both trees.

### Example Benchmark Results

```sh
Starting verkle benchmark for 65536 keys.
Run 0: Proving time: 123ms, Verification time: 56ms, Proof size: 2340 bytes
Run 1: Proving time: 128ms, Verification time: 52ms, Proof size: 2320 bytes
...
Finished benchmarking verkle.

Starting binary benchmarking with 64 keys and 10 runs.
Run 0: Proving time: 34ms, Verification time: 20ms, Proof size: 1500 bytes
Run 1: Proving time: 36ms, Verification time: 22ms, Proof size: 1520 bytes
...
```

## Customization

You can adjust the number of keys to benchmark and the number of runs by modifying the `NUM_KEYS` and `num_runs` parameters in the source code.

### Benchmarking on Different Machines

The benchmarking framework supports execution on different hardware configurations. If you'd like to benchmark on multiple machines, make sure to update the table in the `README` with the appropriate system specifications.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## References

- [Verkle Trees in Rust (rust-verkle)](https://github.com/crate-crypto/rust-verkle)
- [Poseidon Merkle Tree](https://github.com/dusk-network/merkle)
- [Dusk Plonk](https://github.com/dusk-network/plonk)

## Contact

For any questions, feel free to reach out to [jan.oberst@student.kit.edu](mailto:jan.oberst@student.kit.edu).
