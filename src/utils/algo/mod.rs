pub mod recursive;
pub mod search;
pub mod sort;

use rand::seq::SliceRandom;

use super::gen_range_signed;

#[allow(dead_code)]
pub fn run(n: usize, logger: bool) {
    let mut vec = Vec::<isize>::with_capacity(n);
    for value in gen_range_signed(n) {
        vec.push(value);
    }
    vec.shuffle(&mut rand::thread_rng());

    sort::run(&mut vec, logger);
    search::run(&vec, logger);
    recursive::run(logger);
}
