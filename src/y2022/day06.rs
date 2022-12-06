use std::string::ParseError;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn parse_input_day1(input: &str) -> Result<String, ParseError> {
    Ok(input.to_owned())
}

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    calc_index(input, 4)
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    calc_index(input, 14)
}

// ---------------------------- HELPERS ----------------------------

fn distinct(window: &[&u8]) -> bool {
    !(1..window.len()).any(|i| window[i..].contains(&window[i - 1]))
}

fn calc_index(input: &str, window_length: usize) -> usize {
    let mut input = input.as_bytes().iter();
    let mut index = window_length;
    let mut window = vec![];
    for _ in 0..window_length {
        window.push(input.next().unwrap());
    }
    while !distinct(&window) {
        window.rotate_left(1);
        window[3] = input.next().unwrap();
        index += 1;
    }
    index
}

// --------------------- ALTERNATIVE SOLUTIONS ---------------------

#[aoc(day6, part1, alt)]
fn part1_alt(input: &str) -> usize {
    calc_index_alt(input, 4)
}

#[aoc(day6, part2, alt)]
fn part2_alt(input: &str) -> usize {
    calc_index_alt(input, 14)
}

fn distinct_alt(window: &[&u8]) -> bool {
    for (i, a) in window.iter().enumerate() {
        for b in window[(i + 1)..].iter() {
            if a == b {
                return false;
            }
        }
    }
    true
}

fn calc_index_alt(input: &str, window_length: usize) -> usize {
    let mut input = input.as_bytes().iter();
    let mut index = window_length;
    let mut window = vec![];
    for _ in 0..window_length {
        window.push(input.next().unwrap());
    }
    while !distinct_alt(&window) {
        window.rotate_left(1);
        window[3] = input.next().unwrap();
        index += 1;
    }
    index
}
