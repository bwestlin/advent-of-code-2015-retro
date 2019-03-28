extern crate time;

use time::*;

pub fn measure<F>(f: F) where F: FnOnce() -> () {
    let start = precise_time_ns();

    f();

    let dur_ns = precise_time_ns() - start;
    println!("It took: {}ms", dur_ns as f64 / 1_000_000.0);
}

pub fn measure_times<F>(times: usize, f: F) where F: Fn() -> () {
    let start = precise_time_ns();

    for _ in 0..times {
        f();
    }

    let dur_ns = precise_time_ns() - start;
    println!("It took: {}ms on average for {} times", (dur_ns / times as u64) as f64 / 1_000_000.0, times);
}
