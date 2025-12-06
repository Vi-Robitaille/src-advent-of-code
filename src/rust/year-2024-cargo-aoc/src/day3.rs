use aoc_runner_derive::{aoc, aoc_generator};

use nom::{
    bytes::complete::{tag, take_while},
    character::complete::char,
    combinator::map_res,
    sequence::{delimited, separated_pair},
    IResult,
};

type Inp = Vec<String>;

// mul(8,5)

fn from_b10(input: &str) -> Result<usize, std::num::ParseIntError> {
    input.parse::<usize>()
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn num_extraction(input: &str) -> IResult<&str, usize> {
    map_res(take_while(is_digit), from_b10)(input)
}

fn parens(input: &str) -> IResult<&str, (usize, usize)> {
    delimited(
        tag("mul("),
        separated_pair(num_extraction, char(','), num_extraction),
        char(')'),
    )(input)
}

fn check_enable(input: &str) -> IResult<&str, &str> {
    tag("do()")(input)
}
fn check_disable(input: &str) -> IResult<&str, &str> {
    tag("don't()")(input)
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Inp {
    input.lines().map(|x| x.to_string()).collect()
}

#[aoc(day3, part1)]
fn part_one(input: &Inp) -> usize {
    let mut total = 0;
    for i in input {
        for j in 0..i.len() {
            if let Ok((_, (a, b))) = parens(&i[j..]) {
                total += a * b;
            }
        }
    }
    total
}

#[aoc(day3, part2)]
fn part_two(input: &Inp) -> usize {
    let mut total = 0;
    let mut status = true;
    for i in input {
        for j in 0..i.len() {
            let test_pattern = &i[j..];
            if check_enable(test_pattern).is_ok() {
                status = true;
            }
            if check_disable(test_pattern).is_ok() {
                status = false;
            }
            if let (Ok((_, (a, b))), true) = (parens(test_pattern), status) {
                total += a * b;
            }
        }
    }
    total
}
