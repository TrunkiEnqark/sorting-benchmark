//! This module contains implementations of various sorting algorithms.

/// Bubble sort implementation
pub fn bubble_sort<T: Ord + Copy>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

/// Selection sort implementation
pub fn selection_sort<T: Ord + Copy>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        let mut min_idx = i;
        for j in i + 1..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        arr.swap(i, min_idx);
    }
}

/// Insertion sort implementation
pub fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
    let n = arr.len();
    for i in 1..n {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_sorted<T: Ord>(arr: &[T]) -> bool {
        arr.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn test_bubble_sort() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        bubble_sort(&mut arr);
        assert!(is_sorted(&arr));
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_selection_sort() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        selection_sort(&mut arr);
        assert!(is_sorted(&arr));
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_insertion_sort() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
} 