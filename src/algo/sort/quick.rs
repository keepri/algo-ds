use std::time::Instant;

use crate::utils;

fn partition(arr: &mut Vec<isize>, lo: usize, hi: usize) -> usize {
    let pivot_idx = hi;
    let pivot = arr[pivot_idx];
    let mut index = lo;

    for i in lo..hi {
        if arr[i] < pivot {
            arr.swap(i, index);
            index += 1;
        }
    }
    arr.swap(pivot_idx, index);

    return index;
}

fn qs(arr: &mut Vec<isize>, lo: usize, hi: usize) -> () {
    if lo >= hi {
        return;
    }

    let pivot_idx = partition(arr, lo, hi);
    if pivot_idx > 0 {
        qs(arr, lo, pivot_idx - 1);
    }
    qs(arr, pivot_idx + 1, hi);
}

pub fn sort(arr: &mut Vec<isize>, logger: bool) -> () {
    let start_time = Instant::now();

    qs(arr, 0, &arr.len() - 1);

    if logger == true {
        println!(
            "qsort finished in {}s",
            utils::parse_duration(Instant::now().duration_since(start_time))
        );
    }
}
