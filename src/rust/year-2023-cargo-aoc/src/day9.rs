use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;

#[aoc_generator(day9)]
fn parse_input_day1(input: &str) -> Vec<Vec<isize>> {
    input.lines()
        .map(|l| {
            l.split(" ")
                .map(|x| x.parse::<isize>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<isize>>>()
}

#[aoc(day9, part1)]
fn part_one(input: &Vec<Vec<isize>>) -> usize {
    println!("{:?}", input.first().unwrap());
    1
}

#[aoc(day9, part2)]
fn part_two(input: &Vec<Vec<isize>>) -> usize {
    1
}

fn find_difference(inp: Vec<isize>) -> isize {
    let diffs = inp
        .windows(2)
        .map(|x| x[1] - x[0])
        .collect::<Vec<isize>>();

    1
}