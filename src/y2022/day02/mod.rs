use fileinput::FileInput;
use std::io::{BufRead, BufReader};

pub fn task1() -> String {
    let filenames = vec!["./assignments/y2022/day02"];
    let fileinput = FileInput::new(&filenames);
    let reader = BufReader::new(fileinput);

    let rock = 1;
    let paper = 2;
    let scissors = 3;
    let loss = 0;
    let draw = 3;
    let win = 6;

    let mut sum = 0;
    for line in reader.lines() {
        let line: &str = &line.unwrap();
        sum += match line {
            "A X" => draw + rock,
            "A Y" => win + paper,
            "A Z" => draw + scissors,
            "B X" => loss + rock,
            "B Y" => draw + paper,
            "B Z" => win + scissors,
            "C X" => win + rock,
            "C Y" => loss + paper,
            "C Z" => draw + scissors,
            _ => 0,
        };
    }
    sum.to_string()
}

pub fn task2() -> String {
    let filenames = vec!["./assignments/y2022/day02"];
    let fileinput = FileInput::new(&filenames);
    let reader = BufReader::new(fileinput);

    let rock = 1;
    let paper = 2;
    let scissors = 3;
    let loss = 0;
    let draw = 3;
    let win = 6;

    let mut sum = 0;
    for line in reader.lines() {
        let line: &str = &line.unwrap();
        sum += match line {
            "A X" => loss + scissors,
            "B X" => loss + rock,
            "C X" => loss + paper,
            "A Y" => draw + rock,
            "B Y" => draw + paper,
            "C Y" => draw + scissors,
            "A Z" => win + paper,
            "B Z" => win + scissors,
            "C Z" => win + rock,
            _ => 0,
        };
    }
    sum.to_string()
}
