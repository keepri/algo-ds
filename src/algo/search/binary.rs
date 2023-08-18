use std::time::Instant;

use crate::utils;

pub fn search<'a>(arr: &'a Vec<isize>, search: isize, logger: bool) -> Option<&'a isize> {
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

    if logger == true {
        match found {
            Some(value) => println!(
                "binary search found {} in {}s",
                value,
                utils::parse_duration(Instant::now().duration_since(start_time))
            ),
            None => println!("binary search did not find the value"),
        }
    }

    fn calc_middle(low: usize, high: usize) -> usize {
        return (high + low) / 2;
        // return low + (high - low) / 2;
    }

    return found;
}
