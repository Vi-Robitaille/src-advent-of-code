use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

use lazy_static::lazy_static;

lazy_static! {
    static ref GAME_REGEX: Regex =
        Regex::new(r"Game (?P<game_id>\d+):(?P<hand>( \d+ (red|blue|green),?)+;?)+").unwrap();
    static ref RED_REGEX: Regex = Regex::new(r"(?P<item_count>\d+) red").unwrap();
    static ref GRE_REGEX: Regex = Regex::new(r"(?P<item_count>\d+) green").unwrap();
    static ref BLU_REGEX: Regex = Regex::new(r"(?P<item_count>\d+) blue").unwrap();
}

#[aoc_generator(day2)]
fn parse_input_day1(input: &str) -> Vec<Game> {
    input.lines().map(Game::from).collect::<Vec<Game>>()
}

#[aoc(day2, part1)]
fn part_one(input: &[Game]) -> usize {
    let constraints = Hand {
        red: 12,
        green: 13,
        blue: 14,
    };
    input
        .iter()
        .filter(|f| f.is_valid(&constraints))
        .map(|f| f.id)
        .sum()
}

#[aoc(day2, part2)]
fn part_two(input: &[Game]) -> usize {
    input.iter().map(|f| f.power_level()).sum()
}

struct Game {
    id: usize,
    dealings: Vec<Hand>,
}

impl From<&str> for Game {
    // Example Game
    // Game 9: 1 green, 5 blue; 4 blue; 2 red, 1 blue
    fn from(value: &str) -> Self {
        // Im hungover fuck you
        let mut spl_game = value.split(':');
        let id = spl_game
            .next()
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let dealings = spl_game
            .next()
            .unwrap()
            .split(';')
            .map(Hand::from)
            .collect::<Vec<Hand>>();
        Game { id, dealings }
    }
}

impl Game {
    fn is_valid(&self, constraints: &Hand) -> bool {
        self.dealings.iter().all(|d| d.is_valid(constraints))
    }

    fn power_level(&self) -> usize {
        let bigliest_red = self
            .dealings
            .iter()
            .max_by(|x, y| x.red.cmp(&y.red))
            .unwrap()
            .red;
        let bigliest_green = self
            .dealings
            .iter()
            .max_by(|x, y| x.green.cmp(&y.green))
            .unwrap()
            .green;
        let bigliest_blue = self
            .dealings
            .iter()
            .max_by(|x, y| x.blue.cmp(&y.blue))
            .unwrap()
            .blue;
        bigliest_red * bigliest_green * bigliest_blue
    }
}

struct Hand {
    red: usize,
    green: usize,
    blue: usize,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let red = if let Some(caps) = RED_REGEX.captures(value) {
            caps["item_count"].parse::<usize>().unwrap()
        } else {
            0
        };
        let green = if let Some(caps) = GRE_REGEX.captures(value) {
            caps["item_count"].parse::<usize>().unwrap()
        } else {
            0
        };
        let blue = if let Some(caps) = BLU_REGEX.captures(value) {
            caps["item_count"].parse::<usize>().unwrap()
        } else {
            0
        };
        Hand { red, green, blue }
    }
}

impl Hand {
    fn is_valid(&self, constraints: &Hand) -> bool {
        self.red <= constraints.red
            && self.green <= constraints.green
            && self.blue <= constraints.blue
    }
}
