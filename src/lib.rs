//! A library for comparing sorting algorithms performance.

pub mod sorts;
pub mod benchmark;

// Re-export the sorting algorithms
pub use sorts::{bubble_sort, selection_sort, insertion_sort};