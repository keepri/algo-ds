use rand::{seq::SliceRandom, Rng};

use crate::{
    queue::Queue,
    recursion, search,
    sort::{self, Direction},
    stack::Stack,
};
use std::time::Duration;

pub fn parse_duration(duration: Duration) -> f64 {
    return duration.as_secs_f64();
}

pub fn run_ds(n: usize, logger: bool) -> () {
    run_queue(n, logger);
    run_stack(n, logger);
}

pub fn run_algo(n: usize, logger: bool) {
    let mut vec = Vec::<isize>::with_capacity(n);
    let half_n: isize = n as isize / 2;
    let range = -half_n..half_n;

    for value in range.clone() {
        vec.push(value);
    }
    vec.shuffle(&mut rand::thread_rng());

    run_sort(&mut vec, logger);
    run_search(&vec, logger);
    run_recursive(logger);
}

fn run_sort(vec: &mut Vec<isize>, logger: bool) {
    if logger == true {
        println!();
        println!("-------------------------------------------- Algorithms ---------------------------------------------");
        println!("|                                               Sort                                                |");
        println!("-----------------------------------------------------------------------------------------------------");
        println!("{} items", vec.len());
    }
    sort::bubble_sort(&mut vec.clone(), Some(Direction::Desc), logger);
    sort::quick_sort(&mut vec.clone(), logger);
    sort::insertion_sort(vec, logger);
}

fn run_search(vec: &Vec<isize>, logger: bool) {
    let half_n: isize = vec.len() as isize / 2;
    let range = -half_n..half_n;
    let to_find = rand::thread_rng().gen_range(range);

    if logger == true {
        println!();
        println!("-------------------------------------------- Algorithms ---------------------------------------------");
        println!("|                                              Search                                               |");
        println!("-----------------------------------------------------------------------------------------------------");
        println!("{} items", vec.len());
    }
    search::linear_search(&vec, to_find, logger);
    search::binary_search(&vec, to_find, logger);
    search::two_crystal_balls(
        &vec![false, false, false, false, false, false, false, true],
        logger,
    );
}

fn run_recursive(logger: bool) {
    if logger == true {
        println!();
        println!("-------------------------------------------- Algorithms ---------------------------------------------");
        println!("|                                            Recursive                                              |");
        println!("-----------------------------------------------------------------------------------------------------");
    }
    recursion::solve_maze_recursively(logger);
}

fn run_queue(n: usize, logger: bool) -> () {
    if logger == true {
        println!();
        println!("------------------------------------------ Data Structures ------------------------------------------");
        println!("|                                               Queue                                               |");
        println!("-----------------------------------------------------------------------------------------------------");
    }

    let mut queue: Queue<usize> = Queue::new();

    for i in 1..=n {
        queue.enqueue(i);
    }

    if logger == true {
        println!("initialized");
        println!("enqueued 1, 2 to queue");
        println!("len is {}", queue.len());
    }

    if let Some(value) = queue.peek() {
        if logger == true {
            println!("peeked {}", value);
        }
    };

    if let Some(value) = queue.dequeue() {
        if logger == true {
            println!("dequeued {}", value);
            println!("len is {}", queue.len());
        }
    }

    if let Some(value) = queue.peek() {
        if logger == true {
            println!("peeked {}", value);
        }
    };

    if let Some(value) = queue.dequeue() {
        if logger == true {
            println!("dequeued {}", value);
            println!("len is {}", queue.len());
        }
    }
}

fn run_stack(n: usize, logger: bool) -> () {
    if logger == true {
        println!();
        println!("------------------------------------------ Data Structures ------------------------------------------");
        println!("|                                               Stack                                               |");
        println!("-----------------------------------------------------------------------------------------------------");
    }

    let mut stack: Stack<usize> = Stack::new();

    for i in 1..=n {
        stack.push(i);
    }

    if logger == true {
        println!("initialized");
        println!("pushed {n} items to stack");
        println!("len is {}", stack.len());
    }

    if let Some(value) = stack.peek() {
        if logger == true {
            println!("peeked {}", value);
        }
    };

    if let Some(value) = stack.pop() {
        if logger == true {
            println!("popped {}", value);
            println!("len is {}", stack.len());
        }
    }

    if let Some(value) = stack.peek() {
        if logger == true {
            println!("peeked {}", value);
        }
    };

    if let Some(value) = stack.pop() {
        if logger == true {
            println!("popped {}", value);
            println!("len is {}", stack.len());
        }
    }
}
