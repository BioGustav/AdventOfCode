use aoc_runner_derive::{aoc, aoc_generator};
use std::{num::ParseIntError, ops::AddAssign, str::FromStr};

const MAX_MINUTES: i32 = 24;

#[aoc_generator(day19)]
fn parse_input_day19(input: &str) -> Result<Vec<Blueprint>, ParseIntError> {
    let mut blueprints: Vec<Blueprint> = vec![];
    for line in input.lines() {
        let split: Vec<&str> = line.split(' ').collect();
        let id = FromStr::from_str(&split[1][0..split[1].len() - 1])?;
        let ore = FromStr::from_str(split[6])?;
        let clay = FromStr::from_str(split[12])?;
        let obsidian = (FromStr::from_str(split[18])?, FromStr::from_str(split[21])?);
        let geode = (FromStr::from_str(split[27])?, FromStr::from_str(split[30])?);

        let blueprint = Blueprint::new(id, ore, clay, obsidian, geode);
        blueprints.push(blueprint);
    }

    Ok(blueprints)
}

#[aoc(day19, part1)]
fn part1(input: &[Blueprint]) -> u32 {
    let mut sum = 0;
    let mut stock = Stock::init();
    let mut robots = Robots::init();
    for blueprint in input {
        stock = Stock::init();
        robots = Robots::init();
        for minute in 1..=MAX_MINUTES {
            stock += robots.into();
        }
        sum += blueprint.id * stock.geode;
    }
    dbg!(stock);
    dbg!(robots);

    sum
}

#[aoc(day19, part2)]
fn part2(input: &[Blueprint]) -> u32 {
    0
}

//---------------------------------- HELPERS ----------------------------------
#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore: u32,
    clay: u32,
    obsidian: (u32, u32),
    geode: (u32, u32),
}
impl Blueprint {
    fn new(id: u32, ore: u32, clay: u32, obsidian: (u32, u32), geode: (u32, u32)) -> Self {
        Self {
            id,
            ore,
            clay,
            obsidian,
            geode,
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct Robots {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Robots {
    fn new(ore: u32, clay: u32, obsidian: u32, geode: u32) -> Self {
        Self {
            ore,
            clay,
            obsidian,
            geode,
        }
    }
    fn init() -> Self {
        Robots::new(1, 0, 0, 0)
    }
}

#[derive(Debug)]
struct Stock {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Stock {
    fn new(ore: u32, clay: u32, obsidian: u32, geode: u32) -> Self {
        Self {
            ore,
            clay,
            obsidian,
            geode,
        }
    }
    fn init() -> Self {
        Stock::new(0, 0, 0, 0)
    }
}

impl From<Robots> for Stock {
    fn from(robots: Robots) -> Self {
        Self::new(robots.ore, robots.clay, robots.obsidian, robots.geode)
    }
}

impl AddAssign for Stock {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}
