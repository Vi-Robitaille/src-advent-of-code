use aoc_runner_derive::{aoc, aoc_generator};

use nom::{
    IResult,
    sequence::delimited,
    // see the "streaming/complete" paragraph lower for an explanation of these submodules
    character::complete::char,
    bytes::complete::{is_not, tag}
};

type Inp = Vec<String>;

// mul(8,5)

fn parens(input: &str) -> IResult<&str, &str> {
    delimited(tag("mul("), second, char(')'))
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Inp {
    input.lines().map(|x| x.to_string()).collect()
}

#[aoc(day3, part1)]
fn part_one(input: &Inp) -> usize {
    1
}

#[aoc(day3, part2)]
fn part_two(input: &Inp) -> usize {
    1
}
