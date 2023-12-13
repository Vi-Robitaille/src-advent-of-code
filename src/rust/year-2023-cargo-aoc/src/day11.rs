use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;




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
fn parse_input_day11(input: &str) -> FinishedUniverse {
    let inp = Universe(input.lines().map(|a| a.chars().collect_vec()).collect_vec());
    let r = FinishedUniverse::from(inp);
    r
}

#[aoc(day11, part1)]
fn part_one(input: &FinishedUniverse) -> u8 {
    1
}

#[aoc(day11, part2)]
fn part_two(input: &FinishedUniverse) -> u8 {
    1
}

struct Point {
    x: usize,
    y: usize,
}


struct FinishedUniverse {
    grid: Universe,
    empty_columns: Vec<usize>,
    empty_rows: Vec<usize>,
    galaxy_coords: Vec<Point>
}

impl From<Universe> for FinishedUniverse {
    fn from(value: Universe) -> Self {
        let v = value.expand_empty_space();
        let empty_rows = v.empty_rows();
        let mut grid = Universe(transpose(v.0));
        grid = grid.expand_empty_space();
        let empty_columns = grid.empty_rows();
        let galaxy_coords = grid.get_galaxy_coords();

        FinishedUniverse {
            grid,
            empty_columns,
            empty_rows,
            galaxy_coords,
        }
    }
}

struct Universe(Vec<Vec<char>>);
impl Universe {
    fn expand_empty_space(mut self) -> Self {
        todo!();
    }
    fn empty_rows(&self) -> Vec<usize> {
        self.0.iter()
            .enumerate()
            .filter_map(|(i, e)| {
                (e.iter().any(|&c| c == '#')).then_some(i)
            })
            .collect_vec()
    }
    fn get_galaxy_coords(&self) -> Vec<Point> {
        let mut res = vec![];
        for (x, i) in self.0.iter().enumerate() {
            for (y, j) in i.iter().enumerate() {
                if *j == '#' {
                    res.push(Point { x, y })
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