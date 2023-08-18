use crate::ds::stack::Stack;

pub fn run(n: usize, logger: bool) -> () {
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
        println!("initialized, pushed {n} items");
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

    stack.drop();
}
