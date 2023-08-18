use crate::algo::sort::{bubble, insertion, quick, Direction};

pub fn run(vec: &mut Vec<isize>, logger: bool) {
    if logger == true {
        println!();
        println!("-------------------------------------------- Algorithms ---------------------------------------------");
        println!("|                                               Sort                                                |");
        println!("-----------------------------------------------------------------------------------------------------");
        println!("{} items", vec.len());
    }
    bubble::sort(vec, Some(Direction::Desc), logger);
    quick::sort(vec, logger);
    insertion::sort(vec, logger);
}
