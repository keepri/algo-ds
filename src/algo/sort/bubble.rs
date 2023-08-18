use std::time::Instant;

use super::Direction;
use crate::utils;

pub fn sort(arr: &mut Vec<isize>, direction: Option<Direction>, logger: bool) -> () {
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
                Direction::Asc => "ascending",
                Direction::Desc => "descending",
            }
        );
    }
}
