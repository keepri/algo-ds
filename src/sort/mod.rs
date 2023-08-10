use std::time::Instant;

use crate::{sort, utils};

pub struct SortConfig {
    pub logger: bool,
    pub direction: Option<Direction>,
}

#[derive(PartialEq)]
pub enum Direction {
    Asc,
    Desc,
}

pub fn bubble_sort(arr: &mut Vec<isize>, config: Option<SortConfig>) -> () {
    let start_time = Instant::now();

    let mut bubble: isize;
    let mut direction = Direction::Asc;
    let is_asc = direction == Direction::Asc;
    let mut logger: bool = false;
    if let Some(config) = config {
        logger = config.logger;
        if let Some(cfg_dir) = config.direction {
            direction = cfg_dir;
        }
    }

    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if (is_asc && arr[j] > arr[j + 1]) || (!is_asc && arr[j] < arr[j + 1]) {
                bubble = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = bubble;
            }
        }
    }

    let end_time = Instant::now();
    if logger == true {
        println!(
            "bsort finished in {}s - {}",
            utils::parse_duration(end_time.duration_since(start_time)),
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

pub fn quick_sort(arr: &mut Vec<isize>, config: Option<SortConfig>) -> () {
    let start_time = Instant::now();
    let len = arr.len();
    qs(arr, 0, len - 1);

    let end_time = Instant::now();
    if let Some(config) = config {
        if config.logger == true {
            println!(
                "qsort finished in {}s",
                utils::parse_duration(end_time.duration_since(start_time))
            );
        }
    }
}

pub fn insertion_sort(arr: &mut Vec<isize>, config: Option<SortConfig>) -> () {
    let start_time = Instant::now();

    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }

    let end_time = Instant::now();
    if let Some(config) = config {
        if config.logger == true {
            println!(
                "isort finished in {}s",
                utils::parse_duration(end_time.duration_since(start_time))
            );
        }
    }
}
