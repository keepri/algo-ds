pub mod algo;
pub mod ds;

use std::{ops::Range, time::Duration};

pub fn parse_duration(duration: Duration) -> f64 {
    return duration.as_secs_f64();
}

pub fn gen_range_signed(n: usize) -> Range<isize> {
    let half_n: isize = n as isize / 2;

    return -half_n..half_n;
}
