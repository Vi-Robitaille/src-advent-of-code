use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Vec<String> {
    input.split('\n').map(String::from).collect::<Vec<String>>()
}

#[aoc(day1, part1)]
fn part_one(input: &[String]) -> usize {
    input
        .iter()
        .map(|x| {
            let left_right = from_one_end(x);
            let right_left = from_one_end(&x.chars().rev().collect::<String>());
            left_right * 10 + right_left
        })
        .sum()
}

#[aoc(day1, part2)]
fn part_two(input: &[String]) -> usize {
    input
        .iter()
        .map(|lamb| {
            let x = &BIG_REPLACE(lamb);
            let y = &x.chars().rev().collect::<String>();
            let left_right = from_one_end(x);
            let right_left = from_one_end(y);
            left_right * 10 + right_left
        })
        .sum()
}

/// Gotta make sure its a yukky solution!
#[allow(non_snake_case)]
fn BIG_REPLACE(inp: &str) -> String {
    inp.replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
        .replace("zero", "zero0zero")
}

fn from_one_end(inp: &str) -> usize {
    for c in inp.chars() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap() as usize;
        }
    }
    0
}
