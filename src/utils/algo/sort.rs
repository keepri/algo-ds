use crate::algo::sort::{bubble, insertion, quick, Direction};

pub fn run(vec: &mut Vec<isize>, logger: bool) {
    if logger == true {
        println!();
        println!("-------------------------------------------- Algorithms ---------------------------------------------");
        println!("|                                               Sort                                                |");
        println!("-----------------------------------------------------------------------------------------------------");
        println!("{} items", vec.len());
    }
    bubble::sort(&mut vec.clone(), Some(Direction::Desc), logger);
    quick::sort(&mut vec.clone(), logger);
    insertion::sort(vec, logger);
}
