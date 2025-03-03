//! Module for benchmarking sorting algorithms.

use rand::Rng;
use std::time::{Duration, Instant};

/// Generate a random array of specified size
pub fn generate_random_array(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0..1000)).collect()
}

/// Benchmark a sorting function with a specific array size
///
/// # Arguments
///
/// * `sort_fn` - The sorting function to benchmark
/// * `array_size` - The size of the array to sort
/// * `iterations` - Number of iterations to run for averaging
///
/// # Returns
///
/// Average duration for the sorting function to complete
pub fn benchmark_sort<F>(sort_fn: F, array_size: usize, iterations: usize) -> Duration
where
    F: Fn(&mut [i32]),
{
    let mut total_duration = Duration::new(0, 0);

    for _ in 0..iterations {
        let mut array = generate_random_array(array_size);
        let start = Instant::now();
        sort_fn(&mut array);
        total_duration += start.elapsed();
    }

    total_duration / iterations as u32
}

/// Run benchmarks for all sorting algorithms and array sizes
///
/// # Arguments
///
/// * `sizes` - Array sizes to benchmark
/// * `iterations` - Number of iterations for each benchmark
///
/// # Returns
///
/// A vector of benchmark results with columns:
/// [array_size, bubble_sort_ms, selection_sort_ms, insertion_sort_ms]
pub fn run_benchmarks(sizes: &[usize], iterations: usize) -> Vec<(usize, u128, u128, u128)> {
    let mut results = Vec::new();

    for &size in sizes {
        let bubble_time = benchmark_sort(crate::bubble_sort, size, iterations);
        let selection_time = benchmark_sort(crate::selection_sort, size, iterations);
        let insertion_time = benchmark_sort(crate::insertion_sort, size, iterations);
        
        results.push((
            size,
            bubble_time.as_millis(),
            selection_time.as_millis(),
            insertion_time.as_millis()
        ));
    }

    results
}

/// Export benchmark results to a CSV file
///
/// # Arguments
///
/// * `results` - Benchmark results to export
/// * `filename` - Name of the CSV file to create
///
/// # Returns
///
/// Result indicating success or failure
pub fn export_results(results: &[(usize, u128, u128, u128)], filename: &str) -> std::io::Result<()> {
    let mut writer = csv::Writer::from_path(filename)?;
    
    // Write header
    writer.write_record(&[
        "array_size", 
        "bubble_sort_ms", 
        "selection_sort_ms", 
        "insertion_sort_ms"
    ])?;
    
    // Write data rows
    for &(size, bubble, selection, insertion) in results {
        writer.write_record(&[
            size.to_string(),
            bubble.to_string(),
            selection.to_string(),
            insertion.to_string()
        ])?;
    }
    
    writer.flush()?;
    Ok(())
}