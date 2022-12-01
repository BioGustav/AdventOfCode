use fileinput::FileInput;
use std::io::{BufRead, BufReader};
fn main() {
    day1_1();
}

fn day1_1() {
    let filenames = vec!["./test/day1_1"];
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
