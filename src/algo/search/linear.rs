use std::time::Instant;

use crate::utils;

pub fn search<'a>(arr: &'a Vec<isize>, search: isize, logger: bool) -> Option<&'a isize> {
    let start_time = Instant::now();
    let mut found: Option<&'a isize> = None;

    for curr in arr {
        if curr == &search {
            found = Some(curr);
            break;
        }
    }

    if logger == true {
        match found {
            Some(value) => println!(
                "linear search found {} in {}s",
                value,
                utils::parse_duration(Instant::now().duration_since(start_time))
            ),
            None => println!("linear search did not find the value"),
        }
    }

    return found;
}
