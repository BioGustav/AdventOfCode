#[derive(Debug)]
pub struct Day {
    pub task1: String,
    pub task2: String,
}
pub trait DayI {
    fn new() -> Day {
        Day {
            task1: Self::task1(),
            task2: Self::task2(),
        }
    }
    fn task1() -> String;
    fn task2() -> String;
}
