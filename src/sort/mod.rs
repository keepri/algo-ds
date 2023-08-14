use std::time::Instant;

use crate::{sort, utils};

#[derive(PartialEq)]
pub enum Direction {
    Asc,
    Desc,
}

pub fn bubble_sort(arr: &mut Vec<isize>, direction: Option<Direction>, logger: bool) -> () {
    let start_time = Instant::now();
    let mut bubble: isize;
    let direction = match direction {
        Some(dir) => dir,
        None => Direction::Asc,
    };
    let is_asc = direction == Direction::Asc;

    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if (is_asc && arr[j] > arr[j + 1]) || (!is_asc && arr[j] < arr[j + 1]) {
                bubble = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = bubble;
            }
        }
    }

    if logger == true {
        println!(
            "bsort finished in {}s - {}",
            utils::parse_duration(Instant::now().duration_since(start_time)),
            match direction {
                sort::Direction::Asc => "ascending",
                sort::Direction::Desc => "descending",
            }
        );
    }
}

// quick sort
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

pub fn quick_sort(arr: &mut Vec<isize>, logger: bool) -> () {
    let start_time = Instant::now();
    let len = arr.len();

    qs(arr, 0, len - 1);

    if logger == true {
        println!(
            "qsort finished in {}s",
            utils::parse_duration(Instant::now().duration_since(start_time))
        );
    }
}

pub fn insertion_sort(arr: &mut Vec<isize>, logger: bool) -> () {
    let start_time = Instant::now();

    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }

    if logger == true {
        println!(
            "isort finished in {}s",
            utils::parse_duration(Instant::now().duration_since(start_time))
        );
    }
}
