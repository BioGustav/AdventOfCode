use aoc_runner_derive::{aoc, aoc_generator};
use std::{
    num::ParseIntError,
    ops::{Add, AddAssign},
    str::FromStr,
};

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
    for blueprint in input {
        println!("Blueprint {}:", blueprint.id);
        let mut stock = Stock::init();
        let mut robots = Robots::init();
        for minute in 1..=MAX_MINUTES {
            println!("== Minute {} ==", minute);
            // Building
            let mut skip = false;
            let mut new_robots = Robots::new(0, 0, 0, 0);
            if fullfilled_multiple(&blueprint.geode, &(stock.ore, stock.obsidian)) {
                new_robots.geode += 1;
                stock.ore -= blueprint.geode.0;
                stock.obsidian -= blueprint.geode.1;
                println!(
                    "Spend {} ore and {} obsidian to start building a geode-cracking robot",
                    blueprint.geode.0, blueprint.geode.1
                );
            }
            if fullfilled_multiple(&blueprint.obsidian, &(stock.ore, stock.clay)) {
                new_robots.obsidian += 1;
                stock.ore -= blueprint.obsidian.0;
                stock.clay -= blueprint.obsidian.1;
                println!(
                    "Spend {} ore and {} clay to start building a obsidian-collecting robot",
                    blueprint.obsidian.0, blueprint.obsidian.1
                );
            }
            if !skip && fullfilled_single(&blueprint.clay, &stock.ore) {
                new_robots.clay += 1;
                stock.ore -= blueprint.clay;
                println!(
                    "Spend {} ore to start building a clay-collecting robot",
                    blueprint.clay
                );
            }
            if !skip && fullfilled_single(&blueprint.ore, &stock.ore) {
                new_robots.ore += 1;
                stock.ore -= blueprint.ore;
                println!(
                    "Spend {} ore to start building a ore-collecting robot",
                    blueprint.ore
                );
            }
            stock += robots.into();
            printRobotsAndStock(&robots, &stock);

            robots += new_robots;
            println!();
        }
        sum += blueprint.id * stock.geode;
        // dbg!(stock);
        // dbg!(robots);
        println!();
    }

    sum
}

#[allow(unused)]
#[aoc(day19, part2)]
fn part2(input: &[Blueprint]) -> u32 {
    0
}

//---------------------------------- HELPERS ----------------------------------
fn printRobotsAndStock(robots: &Robots, stock: &Stock) {
    if robots.ore > 0 {
        println!(
            "{} ore-collecting robot(s) collect(s) {} ore; you now have {} ore.",
            robots.ore, robots.ore, stock.ore
        );
    }
    if robots.clay > 0 {
        println!(
            "{} clay-collecting robot(s) collect(s) {} clay; you now have {} clay.",
            robots.clay, robots.clay, stock.clay
        );
    }
    if robots.obsidian > 0 {
        println!(
            "{} obsidian-collecting robot(s) collect(s) {} obsidian; you now have {} obsidian.",
            robots.obsidian, robots.obsidian, stock.obsidian
        );
    }
    if robots.geode > 0 {
        println!(
            "{} geode-cracking robot(s) crack(s) {} geode; you now have {} open geode(s).",
            robots.geode, robots.geode, stock.geode
        );
    }
}

fn fullfilled_single(mat0: &u32, stock: &u32) -> bool {
    stock >= mat0
}

fn fullfilled_multiple(mat: &(u32, u32), stock: &(u32, u32)) -> bool {
    stock.0 >= mat.0 && stock.1 >= mat.1
}

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

impl AddAssign for Robots {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
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

#[derive(Debug, Clone, Copy)]
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

impl Add for Stock {
    type Output = Stock;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.ore + rhs.ore,
            self.clay + rhs.clay,
            self.obsidian + rhs.obsidian,
            self.geode + rhs.geode,
        )
    }
}
