use std::io::{BufRead, BufReader};

use fileinput::FileInput;

use crate::day::*;

pub struct Day01 {}
impl DayI for Day01 {
    fn task1() -> String {
        let filenames = vec!["./test/day1"];
        let fileinput = FileInput::new(&filenames);
        let reader = BufReader::new(fileinput);

        let mut max = 0;
        let mut sum = 0;
        for line in reader.lines() {
            let text = line.unwrap();
            if text != "" {
                let my_int: i32 = text.to_string().parse().unwrap();
                sum += my_int;
            } else {
                if sum > max {
                    max = sum;
                }
                sum = 0;
            }
        }
        if sum > max {
            max = sum;
        }
        max.to_string()
    }
    fn task2() -> String {
        let filenames = vec!["./test/day1"];
        let fileinput = FileInput::new(&filenames);
        let reader = BufReader::new(fileinput);

        let mut max3 = vec![0; 3];
        let mut sum = 0;
        for line in reader.lines() {
            let text = line.unwrap();
            if text != "" {
                let my_int: i32 = text.to_string().parse().unwrap();
                sum += my_int;
            } else {
                for i in max3.clone() {
                    if sum > i {
                        max3.push(sum);
                        max3.sort();
                        max3.reverse();
                        max3.truncate(3);
                        break;
                    }
                }
                sum = 0;
            }
        }
        sum = 0;
        for i in max3 {
            sum += i;
        }
        sum.to_string()
    }
}

pub struct Day02 {}
impl DayI for Day02 {
    fn task1() -> String {
        todo!()
    }

    fn task2() -> String {
        todo!()
    }
}

pub struct Day03 {}
impl DayI for Day03 {
    fn task1() -> String {
        todo!()
    }

    fn task2() -> String {
        todo!()
    }
}

pub struct Day04 {}
impl DayI for Day04 {
    fn task1() -> String {
        todo!()
    }

    fn task2() -> String {
        todo!()
    }
}

pub struct Day05 {}
impl DayI for Day05 {
    fn task1() -> String {
        todo!()
    }

    fn task2() -> String {
        todo!()
    }
}

pub struct Day06 {}
impl DayI for Day06 {
    fn task1() -> String {
        todo!()
    }

    fn task2() -> String {
        todo!()
    }
}

pub struct Day07 {}
impl DayI for Day07 {
    fn task1() -> String {
        todo!()
    }

    fn task2() -> String {
        todo!()
    }
}

pub struct Day08 {}
impl DayI for Day08 {
    fn task1() -> String {
        todo!()
    }

    fn task2() -> String {
        todo!()
    }
}

pub struct Day09 {}
impl DayI for Day09 {
    fn task1() -> String {
        todo!()
    }

    fn task2() -> String {
        todo!()
    }
}

pub struct Day10 {}
impl DayI for Day10 {
    fn task1() -> String {
        todo!()
    }

    fn task2() -> String {
        todo!()
    }
}

pub struct Day11 {}
impl DayI for Day11 {
    fn task1() -> String {
        todo!()
    }

    fn task2() -> String {
        todo!()
    }
}

pub struct Day12 {}
impl DayI for Day12 {
    fn task1() -> String {
        todo!()
    }

    fn task2() -> String {
        todo!()
    }
}

pub struct Day13 {}
impl DayI for Day13 {
    fn task1() -> String {
        todo!()
    }

    fn task2() -> String {
        todo!()
    }
}

pub struct Day14 {}
impl DayI for Day14 {
    fn task1() -> String {
        todo!()
    }

    fn task2() -> String {
        todo!()
    }
}

pub struct Day15 {}
impl DayI for Day15 {
    fn task1() -> String {
        todo!()
    }

    fn task2() -> String {
        todo!()
    }
}

pub struct Day16 {}
impl DayI for Day16 {
    fn task1() -> String {
        todo!()
    }

    fn task2() -> String {
        todo!()
    }
}

pub struct Day17 {}
impl DayI for Day17 {
    fn task1() -> String {
        todo!()
    }

    fn task2() -> String {
        todo!()
    }
}

pub struct Day18 {}
impl DayI for Day18 {
    fn task1() -> String {
        todo!()
    }

    fn task2() -> String {
        todo!()
    }
}

pub struct Day19 {}
impl DayI for Day19 {
    fn task1() -> String {
        todo!()
    }

    fn task2() -> String {
        todo!()
    }
}

pub struct Day20 {}
impl DayI for Day20 {
    fn task1() -> String {
        todo!()
    }

    fn task2() -> String {
        todo!()
    }
}

pub struct Day21 {}
impl DayI for Day21 {
    fn task1() -> String {
        todo!()
    }

    fn task2() -> String {
        todo!()
    }
}

pub struct Day22 {}
impl DayI for Day22 {
    fn task1() -> String {
        todo!()
    }

    fn task2() -> String {
        todo!()
    }
}

pub struct Day23 {}
impl DayI for Day23 {
    fn task1() -> String {
        todo!()
    }

    fn task2() -> String {
        todo!()
    }
}

pub struct Day24 {}
impl DayI for Day24 {
    fn task1() -> String {
        todo!()
    }

    fn task2() -> String {
        todo!()
    }
}

pub struct Day25 {}
impl DayI for Day25 {
    fn task1() -> String {
        todo!()
    }

    fn task2() -> String {
        todo!()
    }
}
