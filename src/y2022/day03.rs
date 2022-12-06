use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse_input_day3(input: &str) -> Result<String, ParseIntError> {
    Ok(input.to_owned())
}

#[aoc(day3, part1)]
fn part1(rucksacks: &str) -> u32 {
    let mut sum: u32 = 0;
    for rucksack in rucksacks.lines() {
        let (part0, part1) = rucksack.split_at(rucksack.len() / 2);
        sum += get_doubled_item(part0, part1);
    }
    sum
}

#[aoc(day3, part2)]
fn part2(rucksacks: &str) -> u32 {
    let mut lines = rucksacks.lines();
    let mut sum: u32 = 0;
    while let (Some(part1), Some(part2), Some(part3)) = (lines.next(), lines.next(), lines.next()) {
        let part0 = part1;
        let part1 = part2;
        let part2 = part3;
        sum += get_triple_item(&part0, &part1, &part2);
    }
    sum
}

fn to_priority(c: &char) -> u32 {
    match c {
        'a'..='z' => *c as u32 - 'a' as u32 + 1,
        'A'..='Z' => *c as u32 - 'A' as u32 + 27,
        _ => panic!("Unknown character!"),
    }
}

fn get_doubled_item(part0: &str, part1: &str) -> u32 {
    for a in part0.chars() {
        for b in part1.chars() {
            if a == b {
                return to_priority(&a);
            }
        }
    }
    return 0;
}

fn get_triple_item(part0: &str, part1: &str, part2: &str) -> u32 {
    for a in part0.chars() {
        for b in part1.chars() {
            if a == b {
                for c in part2.chars() {
                    if a == c {
                        return to_priority(&a);
                    }
                }
            }
        }
    }
    return 0;
}
