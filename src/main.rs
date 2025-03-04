use sorting_benchmark::benchmark::run_benchmarks;

fn main() {
    println!("Running sorting algorithm benchmarks...");
    
    let sizes = [100, 500, 1000, 2000, 5000, 10000, 100000];
    let iterations = 5;
    
    let results = run_benchmarks(&sizes, iterations);
    
    println!("array_size,bubble_sort_ms,selection_sort_ms,insertion_sort_ms");
    for &(size, bubble, selection, insertion) in &results {
        println!("{},{},{},{}", size, bubble, selection, insertion);
    }
}