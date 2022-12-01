mod day;
mod days;
use day::DayI;

fn main() {
    let day = <days::Day01 as DayI>::new();
    println!("{}, {}", day.task1, day.task2)
}
