use fileinput::FileInput;
use std::io::{BufRead, BufReader};

pub fn task1() -> String {
    let filenames = vec!["./assignments/y2022/day03"];
    let fileinput = FileInput::new(&filenames);
    let reader = BufReader::new(fileinput);

    let mut sum: u32 = 0;
    for line in reader.lines() {
        let rucksack = Rucksack::new(&line.unwrap());
        sum += rucksack.get_doubled_item();
    }
    sum.to_string()
}

pub fn task2() -> String {
    let filenames = vec!["./assignments/y2022/day03"];
    let fileinput = FileInput::new(&filenames);
    let reader = BufReader::new(fileinput);
    let mut lines = reader.lines();

    let mut sum: u32 = 0;

    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        let group = Group::new(line1.unwrap(), line2.unwrap(), line3.unwrap());
        sum += group.get_triple_item();
    }
    return sum.to_string();
}

struct Rucksack(String, String);

impl Rucksack {
    fn new(input: &str) -> Self {
        let index = input.len() / 2;
        let split = input.split_at(index);
        Self(split.0.to_string(), split.1.to_string())
    }
    fn get_doubled_item(&self) -> u32 {
        for a in self.0.chars() {
            for b in self.1.chars() {
                if a == b {
                    return match a {
                        'a'..='z' => a as u32 - 'a' as u32 + 1,
                        'A'..='Z' => a as u32 - 'A' as u32 + 27,
                        _ => 0,
                    };
                }
            }
        }
        return 0;
    }
}

struct Group(String, String, String);
impl Group {
    fn new(i1: String, i2: String, i3: String) -> Self {
        Self(i1, i2, i3)
    }
    fn get_triple_item(&self) -> u32 {
        for a in self.0.chars() {
            for b in self.1.chars() {
                if a == b {
                    for c in self.2.chars() {
                        if a == c {
                            return match a {
                                'a'..='z' => a as u32 - 'a' as u32 + 1,
                                'A'..='Z' => a as u32 - 'A' as u32 + 27,
                                _ => 0,
                            };
                        }
                    }
                }
            }
        }
        return 0;
    }
}
