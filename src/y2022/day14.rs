use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse_input_day1(input: &str) -> Result<Vec<i32>, ParseIntError> {
    // Ok(input
    //     .split("\n\n")
    //     .collect::<Vec<_>>()
    //     .iter()
    //     .map(|group| group.split("\n").collect::<Vec<_>>())
    //     .map(|v| v.iter().map(|&s| s.parse::<i32>().unwrap()).sum())
    //     .collect::<Vec<_>>())

    Ok(vec![0, 0, 0])
}

#[aoc(day2, part1)]
fn part1(calories: &[i32]) -> i32 {
    *calories.into_iter().reduce(|a, b| a.max(b)).unwrap()
}

#[aoc(day2, part2)]
fn part2(calories: &[i32]) -> i32 {
    let mut calories = calories.to_owned();
    calories.sort_by(|a, b| b.cmp(a));
    let top3 = &calories[..3];
    top3.into_iter().sum()
}
