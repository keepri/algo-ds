use crate::ds::queue::Queue;

pub fn run(n: usize, logger: bool) -> () {
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
        println!("initialized, enqueued {n} items");
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

    queue.drop();
}
