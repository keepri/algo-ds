use rand::seq::SliceRandom;

use crate::{ds::heap::min_heap::MinHeap, utils::gen_range_signed};

pub fn run(n: usize, logger: bool) -> () {
    if logger == true {
        println!();
        println!("------------------------------------------ Data Structures ------------------------------------------");
        println!("|                                             Min Heap                                              |");
        println!("-----------------------------------------------------------------------------------------------------");
    }

    let mut heap: MinHeap<isize> = MinHeap::new();

    let mut items = vec![];
    for i in gen_range_signed(n) {
        items.push(i);
    }
    items.shuffle(&mut rand::thread_rng());
    for item in items {
        heap.insert(item);
    }

    if logger == true {
        println!("initialized");
        println!("pushed {n} items");
        println!("len is {}", heap.len());
    }

    let value = heap.peek();
    if logger == true {
        println!("peeked {}", value);
    }

    if let Some(value) = heap.delete() {
        if logger == true {
            println!("popped {}", value);
            println!("len is {}", heap.len());
        }
    }

    let value = heap.peek();
    if logger == true {
        println!("peeked {}", value);
    }

    if let Some(value) = heap.delete() {
        if logger == true {
            println!("popped {}", value);
            println!("len is {}", heap.len());
        }
    }
}
