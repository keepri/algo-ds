use rand::{seq::SliceRandom, Rng};
use std::env::args;

use crate::{
    search::SearchConfig,
    sort::{Direction, SortConfig},
};

mod list;
mod queue;
mod recursion;
mod search;
mod sort;
mod stack;
mod utils;

fn main() {
    const DEFAULT_N: usize = 25_000;

    let mut rng = rand::thread_rng();
    let mut args = args();
    args.next();

    let mut n: usize = DEFAULT_N;
    if let Some(input) = args.next() {
        if let Ok(arg) = input.parse::<usize>() {
            n = arg;
        }
    }

    // vec generation
    let mut vec = Vec::<isize>::with_capacity(n);
    let half_n: isize = n as isize / 2;
    let range = -half_n..half_n;
    for value in range.clone() {
        vec.push(value);
    }
    vec.shuffle(&mut rng);

    println!();
    println!("Hello, algorithms! Processing {n} items...");
    println!();
    println!("sort:");
    sort::bubble_sort(
        &mut vec.clone(),
        Some(SortConfig {
            logger: true,
            direction: Some(Direction::Desc),
        }),
    );
    sort::quick_sort(
        &mut vec.clone(),
        Some(SortConfig {
            logger: true,
            direction: None,
        }),
    );
    sort::insertion_sort(
        &mut vec,
        Some(SortConfig {
            logger: true,
            direction: None,
        }),
    );

    println!();
    println!("search:");
    let to_find = rng.gen_range(range);
    search::linear_search(&vec, to_find, Some(SearchConfig { logger: true }));
    search::binary_search(&vec, to_find, Some(SearchConfig { logger: true }));
    search::two_crystal_balls(
        &vec![false, false, false, false, false, false, false, true],
        Some(SearchConfig { logger: true }),
    );
}
