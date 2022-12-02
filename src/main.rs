// extern crate aoc2022;
// extern crate aoc_runner;
// extern crate aoc_runner_derive;

// use aoc_runner_derive::aoc_main;

// aoc_main! { lib = aoc2022 }

mod y2022;
use std::time::Instant;

use crate::y2022::*;

fn main() {
    let now = Instant::now();
    println!("{} : {}", day02::task1(), day02::task2());
    let elapsed = now.elapsed();
    println!("{:?}", elapsed);
}
