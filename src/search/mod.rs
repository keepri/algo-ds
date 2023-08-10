use std::time::Instant;

use crate::utils;

pub struct SearchConfig {
    pub logger: bool,
}

pub fn linear_search<'a>(
    arr: &'a Vec<isize>,
    search: isize,
    config: Option<SearchConfig>,
) -> Option<&'a isize> {
    let start_time = Instant::now();
    let mut found: Option<&'a isize> = None;

    for curr in arr {
        if curr == &search {
            found = Some(curr);
            break;
        }
    }

    if let Some(config) = config {
        if config.logger == true {
            match found {
                Some(value) => println!(
                    "linear search found {} in {}s",
                    value,
                    utils::parse_duration(Instant::now().duration_since(start_time))
                ),
                None => println!("linear search did not find the value"),
            }
        }
    }

    return found;
}

pub fn binary_search<'a>(
    arr: &'a Vec<isize>,
    search: isize,
    config: Option<SearchConfig>,
) -> Option<&'a isize> {
    let start_time = Instant::now();
    let mut found: Option<&'a isize> = None;
    let mut lo = 0;
    let mut hi = arr.len();
    let mut mid = calc_middle(lo, hi);
    let mut curr = arr[mid];

    while curr != search && hi > lo {
        if curr < search {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }

        mid = calc_middle(lo, hi);

        if mid == arr.len() {
            break;
        }

        curr = arr[mid];
    }

    if curr == search {
        found = Some(&arr[mid]);
    }

    if let Some(config) = config {
        if config.logger == true {
            match found {
                Some(value) => println!(
                    "binary search found {} in {}s",
                    value,
                    utils::parse_duration(Instant::now().duration_since(start_time))
                ),
                None => println!("binary search did not find the value"),
            }
        }
    }

    fn calc_middle(low: usize, high: usize) -> usize {
        return (high + low) / 2;
        // return low + (high - low) / 2;
    }

    return found;
}

pub fn two_crystal_balls(breaks: &Vec<bool>, config: Option<SearchConfig>) -> Option<usize> {
    let jump_amount = (breaks.len() as f64).sqrt().floor() as usize;
    let mut found: Option<usize> = None;

    let mut jump_index = jump_amount;
    for i in (jump_index..breaks.len()).step_by(jump_amount) {
        if breaks[i] {
            jump_index = i;
            break;
        }
    }

    jump_index -= jump_amount;

    for j in jump_index..=breaks.len() {
        if breaks[j] {
            found = Some(j);
            break;
        }
    }

    if let Some(config) = config {
        if config.logger == true {
            println!(
                "two crystal balls: {}",
                match found {
                    Some(index) => format!("breaking point at index {index}"),
                    None => format!("no breaking point found"),
                }
            );
        }
    }

    return found;
}
