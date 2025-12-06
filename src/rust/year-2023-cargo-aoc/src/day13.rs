use aoc_runner_derive::{aoc, aoc_generator};

use crate::helpers::transpose;
use itertools::Itertools;
use std::iter::{zip, Rev, Zip};
use std::ops::{Range, RangeInclusive};

#[aoc_generator(day13)]
fn parse_input(input: &str) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let rows = input
        .split("\n\n")
        .map(|grid| grid.lines().map(str_to_binary).collect_vec())
        .collect_vec();

    let cols = input
        .split("\n\n")
        .map(|grid| {
            // U thot u wer god, no
            //   i am
            let grids = grid.lines().map(|l| l.chars().collect_vec()).collect_vec();
            transpose(grids)
                .iter()
                .map(|l| str_to_binary(&String::from_iter(l)))
                .collect_vec()
        })
        .collect_vec();
    (rows, cols)
}

#[aoc(day13, part1)]
fn part_one(input: &(Vec<Vec<u32>>, Vec<Vec<u32>>)) -> usize {
    zip(&input.0, &input.1)
        .map(|(a, b)| {
            let x = evaluate_grid(a).unwrap_or(0) * 100;
            let y = evaluate_grid(b).unwrap_or(0);
            x + y
        })
        .sum()
}

#[aoc(day13, part2)]
fn part_two(input: &(Vec<Vec<u32>>, Vec<Vec<u32>>)) -> usize {
    zip(&input.0, &input.1)
        .map(|(a, b)| {
            let x = evaluate_grid_second_strike(a, evaluate_grid(a));
            let y = evaluate_grid_second_strike(b, evaluate_grid(b));
            x.unwrap_or(0) * 100 + y.unwrap_or(0)
        })
        .sum()
}

/// Returns valid virror locations
fn evaluate_grid(grid: &[u32]) -> Option<usize> {
    let splits = find_split_points(grid);
    splits
        .iter()
        .filter_map(|&s| {
            let mut rm = required_matches(s, grid.len());
            rm.all(|(a, b)| grid[a] == grid[b]).then_some(s + 1)
        })
        .max()
}

/// Like the other one but not
fn evaluate_grid_second_strike(grid: &[u32], used_split: Option<usize>) -> Option<usize> {
    let splits = find_smudgy_split_points(grid);
    splits
        .iter()
        .filter_map(|&s| {
            if Some(s + 1) == used_split {
                return None;
            }
            let rm = required_matches(s, grid.len());
            let r: usize = rm.map(|(a, b)| differences(grid[a], grid[b])).sum();
            (r == 1).then_some(s + 1)
        })
        .collect_vec()
        .pop()
}

fn str_to_binary(input: &str) -> u32 {
    input
        .chars()
        .map(|x| match x {
            '.' => 0,
            '#' => 1,
            _ => panic!("nuh"),
        })
        .fold(0, |acc, d| (acc << 1) + d)
}

fn find_split_points(input: &[u32]) -> Vec<usize> {
    let mut r: Vec<usize> = vec![];
    for i in 1..input.len() {
        if input[i] == input[i - 1] {
            r.push(i - 1);
        }
    }
    r
}

fn find_smudgy_split_points(input: &[u32]) -> Vec<usize> {
    let mut r: Vec<usize> = vec![];
    for i in 1..input.len() {
        let a = input[i];
        let b = input[i - 1];
        if a == b || differences(a, b) == 1 {
            r.push(i - 1);
        }
    }
    r
}

///
/// Look ma, i can bitshift a bunch
///
fn differences(a: u32, b: u32) -> usize {
    // let mut diffs = (a ^ b).count_ones() as usize;
    // for x in 0..(0b10000000000000000000000000000000u32.trailing_zeros()) {
    //     if a ^ b == (1 << x) {
    //         diffs += 1
    //     }
    // }
    // diffs
    (a ^ b).count_ones() as usize
}

// This thing is gross so we hide it
type Zipped = Zip<Rev<RangeInclusive<usize>>, Range<usize>>;
///
/// Calculate and return the index of required mirror matches
/// ex:
///   (4,5), (3,6), (2, 7), ...
///   
/// This will remove any indexs that are out of bounds
fn required_matches(split_point: usize, grid_len: usize) -> Zipped {
    let a = (0..=split_point).rev();
    let b = (split_point + 1)..grid_len;
    zip(a, b)
}
