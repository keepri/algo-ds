mod algo;
mod ds;
mod utils;

use std::env::args;

const DEFAULT_N: usize = 25_000;

fn main() {
    let mut args = args();
    args.next();

    let mut n: usize = DEFAULT_N;
    if let Some(input) = args.next() {
        if let Ok(arg) = input.parse::<usize>() {
            n = arg;
        }
    }

    let mut logger = true;
    if let Some(arg_log) = args.next() {
        if let Ok(arg_log) = arg_log.parse::<bool>() {
            logger = arg_log;
        };
    }

    utils::algo::run(n, logger);
    utils::ds::run(n, logger);
}
