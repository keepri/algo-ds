use rand::Rng;

use crate::{
    algo::search::{binary, linear, sqrt},
    utils::gen_range_signed,
};

pub fn run(vec: &Vec<isize>, logger: bool) {
    let to_find = rand::thread_rng().gen_range(gen_range_signed(vec.len()));

    if logger == true {
        println!();
        println!("-------------------------------------------- Algorithms ---------------------------------------------");
        println!("|                                              Search                                               |");
        println!("-----------------------------------------------------------------------------------------------------");
        println!("{} items", vec.len());
    }
    linear::search(&vec, to_find, logger);
    binary::search(&vec, to_find, logger);
    sqrt::two_crystal_balls(
        &vec![false, false, false, false, false, false, false, true],
        logger,
    );
}
