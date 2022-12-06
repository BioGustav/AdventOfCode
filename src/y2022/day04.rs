use std::{num::ParseIntError, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn parse_input_day4(input: &str) -> Result<String, ParseIntError> {
    Ok(input.to_owned())
}

#[aoc(day4, part1)]
fn part1(ranges: &str) -> i32 {
    let mut sum = 0;
    for line in ranges.lines() {
        let mut index = 0;
        let mut arr = [0; 4];

        for ass in line.split(",") {
            for ran in ass.split("-") {
                arr[index] = FromStr::from_str(&ran).unwrap();
                index += 1;
            }
        }
        if contains_all(arr[0], arr[1], arr[2], arr[3]) {
            sum += 1;
        }
    }

    sum
}

#[aoc(day4, part2)]
fn part2(ranges: &str) -> i32 {
    let mut sum = 0;
    for line in ranges.lines() {
        let mut index = 0;
        let mut arr = [0; 4];

        for ass in line.split(",") {
            for ran in ass.split("-") {
                arr[index] = FromStr::from_str(&ran).unwrap();
                index += 1;
            }
        }
        if contains_at_least1(arr[0], arr[1], arr[2], arr[3]) {
            sum += 1;
        }
    }

    sum
}

fn contains_all(r0s: i32, r0e: i32, r1s: i32, r1e: i32) -> bool {
    return r0s >= r1s && r0e <= r1e || r1s >= r0s && r1e <= r0e;
}

fn contains_at_least1(r0s: i32, r0e: i32, r1s: i32, r1e: i32) -> bool {
    return !(r0e < r1s || r1e < r0s);
}
