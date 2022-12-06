use std::{num::ParseIntError, str, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse_input_day1(input: &str) -> Result<Craneplace, ParseIntError> {
    let index = input.find("\n\n").unwrap();
    let input = input.split_at(index);

    let numbers = input.0.find('1').unwrap();
    let (chars, numbers) = input.0.split_at(numbers);
    let _number = numbers
        .split_whitespace()
        .map(|s| FromStr::from_str(&s).unwrap())
        .reduce(|a: u32, b: u32| a.max(b))
        .unwrap();
    let mut stacks: Vec<Vec<char>> = Vec::new();

    let chars = chars.trim_end();
    for line in chars.lines().rev() {
        let crates: Vec<_> = line
            .as_bytes()
            .chunks(4)
            .map(|c| c.get(1).unwrap())
            .map(|&s| char::from_u32(s as u32).unwrap())
            .collect();
        for (i, &c) in crates.iter().enumerate() {
            if c != ' ' {
                if let Some(_) = stacks.get(i) {
                    stacks[i].push(c)
                } else {
                    stacks.push(vec![c])
                }
            }
        }
    }
    let moves = input
        .1
        .trim_start()
        .lines()
        .map(|line| {
            let line: Vec<u32> = line
                .split_whitespace()
                .map(|s| FromStr::from_str(&s).unwrap_or_default())
                .collect();
            (line[1], line[3] - 1, line[5] - 1)
        })
        .collect();

    Ok(Craneplace {
        stacks: stacks,
        moves: moves,
    })
}

#[aoc(day5, part1)]
fn part1(input: &Craneplace) -> String {
    let mut stacks = input.stacks.to_owned();
    let moves = input.moves.to_owned();

    for (n_crates, from, to) in moves {
        for _ in 0..n_crates {
            let crate_ = stacks[from as usize].pop().unwrap();
            stacks[to as usize].push(crate_);
        }
    }

    stacks.iter().map(|v| v.last().unwrap()).collect()
}

#[aoc(day5, part2)]
fn part2(input: &Craneplace) -> String {
    let mut stacks = input.stacks.to_owned();
    let moves = input.moves.to_owned();

    for (n_crates, from, to) in moves {
        let mut crates = vec![];
        for _ in 0..n_crates {
            crates.push(stacks[from as usize].pop().unwrap());
        }
        for crate_ in crates.iter().rev() {
            stacks[to as usize].push(*crate_);
        }
    }

    stacks.iter().map(|v| v.last().unwrap()).collect()
}

#[derive(Debug)]
struct Craneplace {
    stacks: Vec<Vec<char>>,
    moves: Vec<(u32, u32, u32)>,
}
