pub mod min_heap;
pub mod queue;
pub mod stack;

#[allow(dead_code)]
pub fn run(n: usize, logger: bool) -> () {
    queue::run(n, logger);
    stack::run(n, logger);
    min_heap::run(n, logger);
}
