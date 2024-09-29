mod verkle;
mod binary;
mod benchmark;
mod utils;

fn main() {
    println!("Starting benchmarks for Verkle and Binary trees.");
    benchmark::benchmark_verkle();
    benchmark::benchmark_binary();
}
