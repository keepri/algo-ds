use crate::algo::recursion::maze;

pub fn run(logger: bool) {
    if logger == true {
        println!();
        println!("-------------------------------------------- Algorithms ---------------------------------------------");
        println!("|                                            Recursive                                              |");
        println!("-----------------------------------------------------------------------------------------------------");
    }
    maze::solve(logger);
}
