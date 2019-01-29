extern crate time;

use time::*;

pub fn measure<F>(f: F) where F: FnOnce() -> () {
    let start = precise_time_ns();

    f();

    let dur_ns = precise_time_ns() - start;
    println!("It took: {}ms", dur_ns / 1_000_000);
}
