use std::time::Instant;

use crate::utils;

pub fn sort(arr: &mut Vec<isize>, logger: bool) -> () {
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
