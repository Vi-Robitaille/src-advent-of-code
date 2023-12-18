use crate::helpers::transpose;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;
use Cell::*;

use std::fmt;

const CYCLES_NEEDED: usize = 1_000_000_000;

#[aoc_generator(day14)]
fn parse_input(input: &str) -> Vec<Vec<Cell>> {
    transpose(
        input
            .lines()
            .map(|s| s.chars().map(|x| Cell::from(x)).collect_vec())
            .collect_vec(),
    )
}

#[aoc(day14, part1)]
fn part_one(input: &[Vec<Cell>]) -> usize {
    let x = roll(input);
    count_system_load(&x)
}

#[aoc(day14, part2)]
fn part_two(input: &[Vec<Cell>]) -> usize {
    let mut x = input.to_owned();
    let mut hm: HashMap<String, usize> = HashMap::new();
    for cycle_id in 0..CYCLES_NEEDED {
        let flatboi = x
            .iter()
            .flatten()
            .fold("".to_string(), |a, b| a + &format!("{}", b));
        if let Some(cycle_count) = hm.insert(flatboi, cycle_id) {
            if (CYCLES_NEEDED - cycle_id) % (cycle_id - cycle_count) == 0 {
                break;
            }
        }
        for _ in 0..4 {
            x = roll(&x);
            x = rotate_cw(x);
        }
    }
    count_system_load(&x)
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
enum Cell {
    Empty,
    Round,
    Cube,
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Empty => write!(f, " "),
            Round => write!(f, "O"),
            Cube => write!(f, "#"),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Empty => write!(f, " "),
            Round => write!(f, "O"),
            Cube => write!(f, "#"),
        }
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Empty,
            'O' => Round,
            '#' => Cube,
            _ => panic!(),
        }
    }
}

fn get_indexes(i: &[Cell], c: &Cell) -> Vec<usize> {
    i.iter()
        .enumerate()
        .filter_map(|(i, e)| (e == c).then_some(i))
        .collect_vec()
}

/// Generates the range of indexes between rocks
/// cubes: The indexes of those rocks
fn get_ranges(cubes: &[usize], system_len: usize) -> Vec<std::ops::Range<usize>> {
    if cubes.len() > 0 {
        let mut r: Vec<usize> = if cubes.contains(&0) { vec![] } else { vec![0] };
        r.extend(cubes);
        r.push(system_len);
        r.windows(2)
            .map(|x| {
                let a = if cubes.contains(&x[0]) {
                    x[0] + 1
                } else {
                    x[0]
                };
                a..x[1]
            })
            .collect_vec()
    } else {
        vec![(0..system_len)]
    }
}

fn rocks_in_range(slice: &[Cell], r: std::ops::Range<usize>) -> usize {
    *slice[r].iter().counts().get(&Round).unwrap_or(&0)
}

// i wrote this before pt 2 ofc, i could re-write it to use the newer stuff for pt2 but eat my shorts
// jk turns out this is kinda broooooooooooooooken
// deal with it
// fn count_system_load(input: &[Vec<Cell>]) -> usize {
//     let system_len = input[0].len();
//     let mut load = 0;
//     for line in input.iter() {
//         // Index of the cubes
//         let cubes = get_indexes(&line, &Cube).iter().map(|x| x + 1).collect_vec();
//         // Index of the round boulders
//         let roundios = get_indexes(&line, &Round);
//         let ranges = get_ranges(&cubes, system_len);
//         for range in ranges.iter() {
//             // println!("[{}, {}]", range.start, range.end);
//             let count = roundios.iter().filter(|x| range.contains(x)).count();
//             load += (range.start..(range.start + count)).fold(0, |acc, b| acc + (system_len - b));
//         }
//     }
//     load
// }

// jk jk i fixed it
fn count_system_load(input: &[Vec<Cell>]) -> usize {
    let mut s = 0;
    for line in input {
        let len = line.len();
        for (y, e) in line.iter().enumerate() {
            if e == &Round {
                s += len - y;
            }
        }
    }
    s
}

fn roll(input: &[Vec<Cell>]) -> Vec<Vec<Cell>> {
    let mut new_value = vec![];
    for line in input {
        let cubes = get_indexes(line, &Cube);
        let ranges = get_ranges(&cubes, line.len());
        let mut new_line: Vec<Cell> = vec![Empty; line.len()];
        for range in ranges {
            let num_rocks = rocks_in_range(&line, range.clone());
            for x in range.start..(range.start + num_rocks) {
                new_line[x] = Round;
            }
        }
        for c in cubes {
            new_line[c] = Cube;
        }
        new_value.push(new_line)
    }
    new_value
}

/// Im a dummy who cant read good 4head
#[allow(dead_code)]
fn rotate_ccw(input: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let mut new_vec = transpose(input);
    new_vec.iter_mut().for_each(|x| x.reverse());
    new_vec
}

fn rotate_cw(input: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let mut new_vec = transpose(input);
    new_vec.reverse();
    new_vec
}
