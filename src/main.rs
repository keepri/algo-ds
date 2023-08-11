use rand::{seq::SliceRandom, Rng};
use std::env::args;

use crate::sort::Direction;

// mod list;
// mod queue;
mod recursion;
mod search;
mod sort;
// mod stack;
mod utils;

const DEFAULT_N: usize = 25_000;

fn main() {
    let mut rng = rand::thread_rng();
    let mut args = args();
    args.next();

    let mut n: usize = DEFAULT_N;
    if let Some(input) = args.next() {
        if let Ok(arg) = input.parse::<usize>() {
            n = arg;
        }
    }

    let mut logger = true;
    if let Some(arg_log) = args.next() {
        if let Ok(arg_log) = arg_log.parse::<bool>() {
            logger = arg_log;
        };
    }

    // vec generation
    let mut vec = Vec::<isize>::with_capacity(n);
    let half_n: isize = n as isize / 2;
    let range = -half_n..half_n;
    for value in range.clone() {
        vec.push(value);
    }
    vec.shuffle(&mut rng);

    if logger == true {
        println!();
        println!("Hello, algorithms! Processing {n} items...");
        println!();
        println!("--------");
        println!("| sort |");
        println!("--------");
    }
    sort::bubble_sort(&mut vec.clone(), Some(Direction::Desc), logger);
    sort::quick_sort(&mut vec.clone(), logger);
    sort::insertion_sort(&mut vec, logger);

    if logger == true {
        println!();
        println!("----------");
        println!("| search |");
        println!("----------");
    }
    let to_find = rng.gen_range(range);
    search::linear_search(&vec, to_find, logger);
    search::binary_search(&vec, to_find, logger);
    search::two_crystal_balls(
        &vec![false, false, false, false, false, false, false, true],
        logger,
    );

    if logger == true {
        println!();
        println!("---------------");
        println!("| recursivity |");
        println!("---------------");
    }
    recursion::solve_maze_recursively(logger);
}
