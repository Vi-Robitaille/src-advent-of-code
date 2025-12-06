use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::iter::zip;

lazy_static! {
    static ref NUMERAL: Regex = Regex::new(r"\d+").unwrap();
}

#[aoc_generator(day6)]
//                                        Race time          Distance
fn parse_input_day1(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut inp_iter = input.lines();
    let race_times = NUMERAL
        .find_iter(inp_iter.next().unwrap())
        .map(|c| c.as_str().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let distances = NUMERAL
        .find_iter(inp_iter.next().unwrap())
        .map(|c| c.as_str().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    (race_times, distances)
}

#[aoc(day6, part1)]
fn part_one(input: &(Vec<usize>, Vec<usize>)) -> usize {
    zip(input.0.clone(), input.1.clone())
        .map(|(race_time, record_distance)| {
            (0..race_time)
                .filter(|s| is_a_win(*s, race_time, record_distance))
                .count()
        })
        .reduce(|a, b| a * b)
        .unwrap()
}

#[aoc(day6, part2)]
fn part_two(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let race_time = input
        .0
        .iter()
        .fold("".to_string(), |acc, x| acc + &x.to_string())
        .parse::<usize>()
        .unwrap();
    let race_dist = input
        .1
        .iter()
        .fold("".to_string(), |acc, x| acc + &x.to_string())
        .parse::<usize>()
        .unwrap();
    (0..race_time)
        .filter(|&s| is_a_win(s, race_time, race_dist))
        .count()
}

fn is_a_win(hold_time: usize, race_time_limit: usize, record_distance: usize) -> bool {
    if hold_time > race_time_limit {
        return false;
    }
    record_distance < (race_time_limit - hold_time) * hold_time
}
