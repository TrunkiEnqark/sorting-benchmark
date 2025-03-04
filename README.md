# Sorting Algorithms Benchmark - Quick Start Guide

A benchmarking project for comparing performance of basic sorting algorithms implemented in Rust.

## Quick Start

### Setup and Run

```bash
# Clone the repository
git clone https://github.com/TrunkiEnqark/sorting-benchmark.git

# Build and run the project 
cargo run

# Run the benchmarks
cargo bench
```

## Benchmark Results

Here are the latest benchmark results comparing the performance of three sorting algorithms:

### Raw Benchmark Data (microseconds)

| Algorithm | 10 elements | 50 elements | 100 elements | 500 elements | 1000 elements |
|-----------|-------------|-------------|--------------|--------------|---------------|
| Bubble Sort | 143.7 ns | 2.587 μs | 7.528 μs | 107.5 μs | 345.2 μs |
| Insertion Sort | 101.8 ns | 501.0 ns | 2.724 μs | 26.43 μs | 83.74 μs |
| Selection Sort | 161.7 ns | 1.525 μs | 5.855 μs | 74.32 μs | 267.3 μs |

### Key Findings:

1. **Insertion Sort** is consistently the fastest algorithm across all input sizes in these benchmarks
2. **Bubble Sort** performs worst, especially as the input size grows
3. **Selection Sort** performs better than Bubble Sort but worse than Insertion Sort

The benchmark was run with 100 samples per algorithm/size combination to ensure accurate results.

## Performance Comparison Chart

```
Insertion Sort: █████████ 83.74 μs
Selection Sort: ██████████████████████████ 267.3 μs  
Bubble Sort:    ████████████████████████████████████ 345.2 μs
```