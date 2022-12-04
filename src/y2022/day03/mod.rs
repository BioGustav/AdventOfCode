use fileinput::FileInput;
use std::io::{BufRead, BufReader};

pub fn task1() -> u32 {
    let filenames = vec!["./assignments/y2022/day03"];
    let fileinput = FileInput::new(&filenames);
    let reader = BufReader::new(fileinput);

    let mut sum: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let (part0, part1) = line.split_at(line.len() / 2);
        sum += get_doubled_item(part0, part1);
    }

    sum
}

pub fn task2() -> u32 {
    let filenames = vec!["./assignments/y2022/day03"];
    let fileinput = FileInput::new(&filenames);
    let reader = BufReader::new(fileinput);

    let mut lines = reader.lines();
    let mut sum: u32 = 0;
    while let (Some(part1), Some(part2), Some(part3)) = (lines.next(), lines.next(), lines.next()) {
        let part0 = part1.unwrap();
        let part1 = part2.unwrap();
        let part2 = part3.unwrap();

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
