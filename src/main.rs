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
    let task1 = day04::task1();
    let elapsed1 = now.elapsed();
    let now = Instant::now();
    let task2 = day04::task2();
    let elapsed2 = now.elapsed();
    println!("{:?} in {:?}", task1, elapsed1);
    println!("{:?} in {:?}", task2, elapsed2);
}
