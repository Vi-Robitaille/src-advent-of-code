use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;
use std::collections::HashSet;

//
// Normally when dealing with a Vec<Vec<T>> x, y indexing can get annoying
//  I'm going to be using x and y as the actual x, y coords as you would see them
//   looking at the input
//

// (0, 0)      (9, 0)
//   A..#.....B
//   .......#..
//   #.........
//   ..........
//   ......#...
//   .#........
//   .........#
//   ..........
//   .......#..
//   C...#....D
// (0, 9)     (9, 9)

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Universe {
    Universe(input.lines().map(|a| a.chars().collect_vec()).collect_vec())
}

#[aoc(day11, part1)]
fn part_one(input: &Universe) -> usize {
    let modifier = 1;
    let universe = FinishedUniverse::new(input, modifier);
    solve(&universe)
}

#[aoc(day11, part2)]
fn part_two(input: &Universe) -> usize {
    let modifier = 1_000_000 - 1;
    let universe = FinishedUniverse::new(input, modifier);
    solve(&universe)
}

fn solve(universe: &FinishedUniverse) -> usize {
    let mut evaluated_points: HashSet<&Point> = HashSet::new();
    universe
        .galaxy_coords
        .iter()
        .map(|lhs| {
            let _ = evaluated_points.insert(lhs);
            universe
                .galaxy_coords
                .iter()
                .map(|rhs| {
                    if evaluated_points.get(rhs).is_none() {
                        lhs.manhat_dist(rhs)
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn dx(&self, rhs: &Self) -> usize {
        match (self.x, rhs.x) {
            (a, b) if a < b => b - a,
            (a, b) if a > b => a - b,
            (a, b) if a == b => 0,
            _ => panic!("How?"),
        }
    }
    fn dy(&self, rhs: &Self) -> usize {
        match (self.y, rhs.y) {
            (a, b) if a < b => b - a,
            (a, b) if a > b => a - b,
            (a, b) if a == b => 0,
            _ => panic!("How?"),
        }
    }
    fn manhat_dist(&self, rhs: &Self) -> usize {
        self.dx(rhs) + self.dy(rhs)
    }
}

struct FinishedUniverse {
    galaxy_coords: Vec<Point>,
}

impl FinishedUniverse {
    fn new(value: &Universe, modifier: usize) -> Self {
        // value.expand_empty_space(1);
        let empty_rows = value.empty_rows();
        let grid = Universe(transpose(value.0.clone()));
        // grid.expand_empty_space(1);
        let empty_cols = grid.empty_rows();
        let galaxy_coords = grid.get_galaxy_coords(&empty_rows, &empty_cols, modifier);

        FinishedUniverse { galaxy_coords }
    }
}

struct Universe(Vec<Vec<char>>);
impl Universe {
    ///
    /// Actually expand space since i have a feeling pt2 is going to pull some shit
    /// in reality tho its gonna need to be solved with math im sure
    ///
    /// Edit.
    /// fuck
    ///
    #[allow(unused)]
    fn expand_empty_space(&mut self, modifier: usize) {
        let mut empty_row_indexes = self.empty_rows();
        empty_row_indexes.sort();

        println!("Found {:?} empty lines", empty_row_indexes.len());
        // Reverse the iter, if we dont the indexes will be invalid at the
        // first insert
        let empty_space = vec!['.'; self.0.len()];
        for i in empty_row_indexes.into_iter().rev() {
            for j in 0..modifier {
                self.0.insert(i + j, empty_space.clone());
            }
        }
    }

    fn empty_rows(&self) -> Vec<usize> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, e)| (!e.iter().any(|&c| c == '#')).then_some(i))
            .collect_vec()
    }

    fn get_galaxy_coords(
        &self,
        empty_rows: &[usize],
        empty_cols: &[usize],
        modifier: usize,
    ) -> Vec<Point> {
        let mut res = vec![];
        for (x, i) in self.0.iter().enumerate() {
            for (y, j) in i.iter().enumerate() {
                if *j == '#' {
                    let x_range = 0..x;
                    let y_range = 0..y;
                    let adx = empty_cols
                        .iter()
                        .filter_map(|a| x_range.contains(a).then_some(modifier))
                        .sum::<usize>();
                    let ady = empty_rows
                        .iter()
                        .filter_map(|a| y_range.contains(a).then_some(modifier))
                        .sum::<usize>();

                    res.push(Point {
                        x: x + adx,
                        y: y + ady,
                    });
                }
            }
        }
        res
    }
}

/// Transpose a vector of vectors 4head
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../input/2023/day11.txt");
    const TEST_INPUT_1: &'static str = include_str!("../input/2023/day11-test.txt");

    #[test]
    fn part_1_test_1() {
        let inp = parse_input(TEST_INPUT_1);
        assert_eq!(part_one(&inp), 374)
    }
    #[test]
    fn part_1_test_2() {
        let inp = parse_input(INPUT);
        assert_eq!(part_one(&inp), 9403026)
    }

    #[test]
    fn part_2_test_1() {
        let inp = parse_input(TEST_INPUT_1);
        let modifier = 10 - 1;
        let universe = FinishedUniverse::new(&inp, modifier);
        assert_eq!(solve(&universe), 1030)
    }

    #[test]
    fn part_2_test_2() {
        let inp = parse_input(TEST_INPUT_1);
        let modifier = 100 - 1;
        let universe = FinishedUniverse::new(&inp, modifier);
        assert_eq!(solve(&universe), 8410)
    }
}
