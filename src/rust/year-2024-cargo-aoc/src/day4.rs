use std::{iter::zip, ops::Range};

use crate::helpers::transpose;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Inp = Vec<Vec<char>>;

const SEARCH: [char; 4] = ['X', 'M', 'A', 'S'];
const SEARCH_REVERSE: [char; 4] = ['S', 'A', 'M', 'X'];

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Inp {
    input.lines().map(|x| x.chars().collect()).collect()
}

#[aoc(day4, part1)]
fn part_one(input: &Inp) -> usize {
    
    let mut count = 0;
    for y in 0..=(input.len() - SEARCH.len()) {
        for x in 0..=(input[y].len() - SEARCH.len()) {
            if input[y][x] == 'X' {
                count += test_section(input, y, x)
            }
        }
    }
    count
}

fn test_section(ss: &Inp, y: usize, x: usize) -> usize {
    let mut count = 0;
    let _t = {
        if let Some(_) = y.checked_sub(SEARCH.len()) {
            let a = [ss[y-3][x], ss[y-2][x], ss[y-1][x], ss[y][x]];
            if a == SEARCH || a == SEARCH_REVERSE {
                count += 1;
            }
        }
    };
    let _tr = {
        if let (Some(_), true) = (y.checked_sub(SEARCH.len()), x + SEARCH.len() <= ss[0].len()) {
            let a = [ss[y-3][x+3], ss[y-2][x+2], ss[y-1][x+1], ss[y][x]];
            if a == SEARCH || a == SEARCH_REVERSE {
                count += 1;
            }
        }
    };
    let _r = {
        if x + SEARCH.len() <= ss[0].len() {
            let a = [ss[y][x+3], ss[y][x+2], ss[y][x+1], ss[y][x]];
            if a == SEARCH || a == SEARCH_REVERSE {
                count += 1;
            }
        }
    };
    let _br = {
        if let (true, true) = (y + SEARCH.len() <= ss.len(), x + SEARCH.len() <= ss[0].len()) {
            let a = [ss[y+3][x+3], ss[y+2][x+2], ss[y+1][x+1], ss[y][x]];
            if a == SEARCH || a == SEARCH_REVERSE {
                count += 1;
            }
        }
    };
    let _b = {
        if y + SEARCH.len() <= ss.len() {
            let a = [ss[y+3][x], ss[y+2][x], ss[y+1][x], ss[y][x]];
            if a == SEARCH || a == SEARCH_REVERSE {
                count += 1;
            }
        }
    };
    // 
    let _tl = {
        if let (Some(_), Some(_)) = (y.checked_sub(SEARCH.len()), x.checked_sub(SEARCH.len())) {
            let a = [ss[y-3][x-3], ss[y-2][x-2], ss[y-1][x-1], ss[y][x]];
            if a == SEARCH || a == SEARCH_REVERSE {
                count += 1;
            }
        }
    };
    let _l = {
        if let Some(_) = x.checked_sub(SEARCH.len()) {
            let a = [ss[y][x-3], ss[y][x-2], ss[y][x-1], ss[y][x]];
            if a == SEARCH || a == SEARCH_REVERSE {
                count += 1;
            }
        }
    };
    let _bl = {
        if let (true, Some(_)) = (y + SEARCH.len() <= ss.len(), x.checked_sub(SEARCH.len())) {
            let a = [ss[y+3][x-3], ss[y+2][x-2], ss[y+1][x-1], ss[y][x]];
            if a == SEARCH || a == SEARCH_REVERSE {
                count += 1;
            }
        }
    };
    count
}

#[aoc(day4, part2)]
fn part_two(input: &Inp) -> usize {
    1
}

// 2571
// 3272 Low
// 3350 high
