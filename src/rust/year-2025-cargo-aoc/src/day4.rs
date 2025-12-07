use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use grid::*;

type Inp = Grid<bool>;

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Inp {
    let lines: Vec<Vec<bool>> = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| y == '@')
                .collect()
        })
        .collect();
    Grid::from(lines)
}

#[aoc(day4, part1)]
fn part_one(input: &Inp) -> usize {
    input
        .indexed_iter()
        .filter(|((row, col), x)| {
            if **x {
                count_neighbors(input, *row, *col) < 4
            } else {
                false
            }
        })
        .count()
}

fn count_neighbors(input: &Inp, row: usize, col: usize) -> usize {
    let up = get_at_index(input, row, col + 1);
    let ur = get_at_index(input, row + 1, col + 1);
    let ri = get_at_index(input, row + 1, col);
    let dr = get_at_index(input, row + 1, col - 1);
    let dw = get_at_index(input, row, col - 1);
    let dl = get_at_index(input, row - 1, col - 1);
    let le = get_at_index(input, row - 1, col);
    let ul = get_at_index(input, row - 1, col + 1);
    [up, ur, ri, dr, dw, dl, le, ul]
        .iter()
        .filter(|x| **x)
        .count()
}

fn get_at_index(input: &Inp, row: usize, col: usize) -> bool {
    input.get(row, col).is_some_and(|x| *x)
}

#[aoc(day4, part2)]
fn part_two(input: &Inp) -> usize {
    let mut removed: HashSet<(usize, usize)> = HashSet::new();

    let mut iter = true;
    while iter {
        iter = input
            .indexed_iter()
            .filter(|((row, col), x)| {
                if **x && !removed.contains(&(*row, *col))
                    && count_neighbors_with_exclusion(input, *row, *col, &removed) < 4 {
                        removed.insert((*row, *col));
                        return true;
                    }
                false
            })
            .count()
            > 0;
    }

    removed.len()
}

fn count_neighbors_with_exclusion(
    input: &Inp,
    row: usize,
    col: usize,
    removed: &HashSet<(usize, usize)>,
) -> usize {
    let up = get_at_index_with_exclusion(input, row, col + 1, removed);
    let ur = get_at_index_with_exclusion(input, row + 1, col + 1, removed);
    let ri = get_at_index_with_exclusion(input, row + 1, col, removed);
    let dr = get_at_index_with_exclusion(input, row + 1, col - 1, removed);
    let dw = get_at_index_with_exclusion(input, row, col - 1, removed);
    let dl = get_at_index_with_exclusion(input, row - 1, col - 1, removed);
    let le = get_at_index_with_exclusion(input, row - 1, col, removed);
    let ul = get_at_index_with_exclusion(input, row - 1, col + 1, removed);
    [up, ur, ri, dr, dw, dl, le, ul]
        .iter()
        .filter(|x| **x)
        .count()
}

fn get_at_index_with_exclusion(
    input: &Inp,
    row: usize,
    col: usize,
    removed: &HashSet<(usize, usize)>,
) -> bool {
    input
        .get(row, col)
        .is_some_and(|x| *x && !removed.contains(&(row, col)))
}
