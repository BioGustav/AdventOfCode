use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Result<Vec<(u32, u32)>, ParseIntError> {
    Ok(input
        .lines()
        .map(|line| line.split_at(1))
        .map(|(a, b)| (to_symbol(a), to_symbol(b)))
        .collect::<Vec<_>>())
}

#[aoc(day2, part1)]
fn part1(rounds: &[(u32, u32)]) -> u32 {
    let mut sum = 0;
    for (o, m) in rounds {
        sum += match (o, m) {
            // lose
            (1, 3) | (2, 1) | (3, 2) => m + 0,
            // draw
            (1, 1) | (2, 2) | (3, 3) => m + 3,
            // win
            (1, 2) | (2, 3) | (3, 1) => m + 6,
            _ => panic!(),
        }
    }
    sum
}

#[aoc(day2, part2)]
fn part2(rounds: &[(u32, u32)]) -> u32 {
    let mut sum = 0;
    for (o, m) in rounds {
        sum += match (o, m) {
            // lose
            (1, 1) | (2, 1) | (3, 1) => to_loss(o) + 0,
            // draw
            (1, 2) | (2, 2) | (3, 2) => o + 3,
            // win
            (1, 3) | (2, 3) | (3, 3) => to_win(o) + 6,
            _ => panic!(),
        }
    }
    sum
}

fn to_symbol(s: &str) -> u32 {
    let s = s.trim();
    let s = s.chars().nth(0).unwrap();
    match s {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => panic!(),
    }
}

fn to_win(c: &u32) -> u32 {
    match c {
        1 => 2,
        2 => 3,
        3 => 1,
        _ => panic!(),
    }
}

fn to_loss(c: &u32) -> u32 {
    match c {
        1 => 3,
        2 => 1,
        3 => 2,
        _ => panic!(),
    }
}
