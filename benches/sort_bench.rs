use divan::{black_box, Bencher};
use rand::seq::SliceRandom;
use sorting_benchmark::{bubble_sort, selection_sort, insertion_sort};

fn main() {
    divan::main();
}

// Array sizes to benchmark
const SIZES: [usize; 5] = [10, 50, 100, 500, 1000];

// Generate a random array for benchmarking
fn random_array(size: usize) -> Vec<i32> {
    let mut array: Vec<i32> = (0..size as i32).collect();
    let mut rng = rand::thread_rng();
    array.shuffle(&mut rng);
    array
}

// Benchmark group for bubble sort
#[divan::bench(consts = SIZES)]
fn bench_bubble_sort<const N: usize>(bencher: Bencher) {
    bencher
        .with_inputs(|| random_array(N))
        .bench_refs(|array| {
            let mut arr = array.clone();
            bubble_sort(black_box(&mut arr));
        });
}

// Benchmark group for selection sort
#[divan::bench(consts = SIZES)]
fn bench_selection_sort<const N: usize>(bencher: Bencher) {
    bencher
        .with_inputs(|| random_array(N))
        .bench_refs(|array| {
            let mut arr = array.clone();
            selection_sort(black_box(&mut arr));
        });
}

// Benchmark group for insertion sort
#[divan::bench(consts = SIZES)]
fn bench_insertion_sort<const N: usize>(bencher: Bencher) {
    bencher
        .with_inputs(|| random_array(N))
        .bench_refs(|array| {
            let mut arr = array.clone();
            insertion_sort(black_box(&mut arr));
        });
}