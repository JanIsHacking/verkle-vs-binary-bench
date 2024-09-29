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

This will generate the results for each tree size, including the proving time, proof size, and verification time for both trees. Note that for now, only the Verkle benchmarks supports benchmarking multiple tree sizes at a time. For the binary trees, you have to adapt the values of `NUM_KEYS`, `KEY_LOG`, and `CAPACITY` manually.

### Example Benchmark Results

Verkle Trees on Machine 1
```sh
name,total_keys,average_proof_time,average_verification_time,average_size,no_runs
verkle,16384,1.8336086s,1.8694769s,714572,10
verkle,65536,1.88775008s,1.53578109s,850668,10
verkle,262144,2.48019581s,1.81236115s,965740,10
verkle,1048576,2.4411082s,1.56143548s,987180,10
verkle,4194304,2.98853826s,1.78451174s,1038284,10
verkle,16777216,51.87884292s,1.60338479s,1169420,10
```

Verkle Trees on Machine 2
```sh
name,total_keys,average_proof_time,average_verification_time,average_size,no_runs
verkle,16384,1.305095922s,0.49779828s,714572,10
verkle,65536,0.972576193s,0.510019633s,850668,10
verkle,262144,1.069842167s,0.533193488s,965740,10
verkle,1048576,1.161895569s,0.549184155s,987180,10
verkle,4194304,1.684242887s,0.55263323s,1038284,10
verkle,16777216,3.17521192s,0.639270327s,1169420,10
verkle,33554432,3.486600325s,0.670396345s,1243660,10
```

Binary Trees on Machine 1
```sh
name,total_keys,average_proof_time,average_verification_time,average_size,no_runs
binary,32,367.28816022s,0.3688766s,32256,10
binary,64,687.951035045s,0.671940376s,64512,10
binary,128,1370.611844626s,1.347063578s,129024,10
```

Binary Trees on Machine 2
```sh
name,total_keys,average_proof_time,average_verification_time,average_size,no_runs
binary,32,226.325181057s,209.162585ms,32256,10
binary,64,452.653555297s,417.485903ms,64512,10
binary,128,904.153645446s,835.34797ms,129024,10
```

## Customization

You can adjust the number of keys to benchmark and the number of runs by modifying the `NUMBER_OF_LEAVES_TO_PROVE` and `NUM_RUNS` parameters in the source code.

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
