use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse_input_day1(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|l| l.split(' ').map(|x| x.parse::<isize>().unwrap()).collect())
        .collect::<Vec<Vec<isize>>>()
}

#[aoc(day9, part1)]
fn part_one(input: &[Vec<isize>]) -> isize {
    input
        .iter()
        .map(|x| {
            let next_val = find_difference(x);
            x.iter().last().unwrap() + next_val
        })
        .sum()
}

#[aoc(day9, part2)]
fn part_two(input: &[Vec<isize>]) -> isize {
    input
        .iter()
        .map(|x| {
            let next_val = find_difference_other_way(x);
            x[0] - next_val
        })
        .sum()
}

macro_rules! differences {
    ($vec:expr) => {{
        let mut diffs = Vec::new();
        if $vec.len() > 1 {
            for i in 1..$vec.len() {
                diffs.push($vec[i] - $vec[i - 1]);
            }
        }
        diffs
    }};
}

fn find_difference(inp: &[isize]) -> isize {
    let mut result: Vec<isize> = Vec::new();
    let mut local_input = inp.to_owned();
    while local_input.windows(2).any(|x| x[1] - x[0] != 0) {
        local_input = differences!(local_input);

        result.push(*local_input.iter().last().unwrap());
    }

    result.iter().sum()
}

fn find_difference_other_way(inp: &[isize]) -> isize {
    let mut result: Vec<isize> = Vec::new();
    let mut local_input = inp.to_owned();
    while local_input.windows(2).any(|x| x[1] - x[0] != 0) {
        local_input = differences!(local_input);

        result.push(local_input[0]);
    }

    result.iter().rev().fold(0, |acc, c| c - acc)
}
