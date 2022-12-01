use fileinput::FileInput;
use std::io::{BufRead, BufReader};
fn main() {
    day1_1();
    day1_2();
}

fn day1_1() {
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
    println!("{}", max);
}

fn day1_2() {
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
    println!("{}", sum);
}
