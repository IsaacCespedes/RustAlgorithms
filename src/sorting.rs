use std::fmt::Debug;
use std::fmt::Display;

pub fn bubble_sort<T: Ord + Debug + Display>(arr: &mut [T]) {
    let n = arr.len();
    let mut swapped;
    // i is the number of elements that are sorted
    // the last i elements are already sorted
    // j is the index of the element that is being compared to the next element
    for i in 0..n {
        swapped = false;
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }

        // If no two elements were swapped in the inner loop, the array is already sorted.
        if !swapped {
            println!("no swap");
            break;
        }
    }
}

pub fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_idx = partition(arr);
    quicksort(&mut arr[0..pivot_idx]);
    quicksort(&mut arr[pivot_idx + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    // there are different ways to choose a pivot index
    // a common way is to choose the middle element
    let pivot_idx = arr.len() / 2;
    arr.swap(pivot_idx, arr.len() - 1);

    // by the time this loop is done,
    // all elements less than the pivot will be on the left
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= arr[arr.len() - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);

    i
}

pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;
    let mut left = arr[0..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    // big o notation: O(n log n)
    // with space complexity of O(n)

    // recursively divides the slice into two halves
    // until the base case of having one or zero elements is reached
    merge_sort(&mut left);
    merge_sort(&mut right);

    // merge the two sorted halves
    merge(&mut left, &mut right, arr);
}

fn merge<T: Ord + Clone>(left: &mut [T], right: &mut [T], arr: &mut [T]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}
