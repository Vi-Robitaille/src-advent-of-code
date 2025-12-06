use aoc_runner_derive::{aoc, aoc_generator};
use rusttype::{point, Point};

use std::collections::HashSet;
use std::ops::Range;

type Inp = Area;

#[derive(Debug, Clone)]
struct Area {
    walls: HashSet<Point<usize>>,
    start: Point<usize>,
    size: (Range<usize>, Range<usize>),
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Inp {
    let mut start = point(0, 0);
    let mut walls = HashSet::new();
    input.lines().enumerate().for_each(|(row, l)| {
        l.chars().enumerate().for_each(|(col, c)| {
            if c == '^' {
                start = point(row, col);
            }
            if c == '#' {
                walls.insert(point(row, col));
            }
        })
    });
    let g: Vec<Vec<char>> = input.lines().map(|c| c.chars().collect()).collect();
    Area {
        walls,
        start,
        size: (0..g.len(), 0..g[0].len()),
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn mutate_coordinate(&self, coordinate: &Point<usize>) -> Option<Point<usize>> {
        use Direction::*;
        match *self {
            Up => {
                if coordinate.x.checked_sub(1).is_some() {
                    return Some(point(coordinate.x - 1, coordinate.y));
                }
                None
            }
            Right => Some(point(coordinate.x, coordinate.y + 1)),
            Down => Some(point(coordinate.x + 1, coordinate.y)),
            Left => {
                if coordinate.y.checked_sub(1).is_some() {
                    return Some(point(coordinate.x, coordinate.y - 1));
                }
                None
            }
        }
    }

    fn next(&self) -> Self {
        use Direction::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

#[aoc(day6, part1)]
fn part_one(input: &Inp) -> usize {
    let mut position = input.start;
    let mut direction = Direction::Up;
    let mut seen: HashSet<Point<usize>> = HashSet::new();
    seen.insert(position);
    loop {
        if let Some(next_step) = direction.mutate_coordinate(&position) {
            if !input.size.0.contains(&next_step.x) || !input.size.1.contains(&next_step.y) {
                break;
            }
            match input.walls.get(&next_step) {
                None => {
                    position = next_step;
                    seen.insert(position);
                }
                Some(_) => {
                    direction = direction.next();
                }
            }
        } else {
            break;
        }
    }
    seen.len()
}

#[aoc(day6, part2)]
fn part_two(input: &Inp) -> usize {
    let mut position = input.start;
    let mut direction = Direction::Up;
    let mut seen: HashSet<(Point<usize>, Direction)> = HashSet::new();
    seen.insert((position, direction));
    loop {
        if let Some(next_step) = direction.mutate_coordinate(&position) {
            if !input.size.0.contains(&next_step.x) || !input.size.1.contains(&next_step.y) {
                break;
            }
            match input.walls.get(&next_step) {
                None => {
                    position = next_step;
                    seen.insert((position, direction));
                }
                Some(_) => {
                    direction = direction.next();
                }
            }
        } else {
            break;
        }
    }
    seen.iter()
        .filter(|(original_position, original_direction)| {
            process_points(input, original_position, original_direction)
        })
        .count()
}

fn process_points(
    input: &Area,
    original_position: &Point<usize>,
    original_direction: &Direction,
) -> bool {
    let mut input = input.clone();
    if let Some(next_step) = original_direction.mutate_coordinate(&original_position) {
        input.walls.insert(next_step);
    } else {
        return false;
    }
    let (mut position, mut direction) = (original_position.clone(), original_direction.clone());
    let mut seen: HashSet<(Point<usize>, Direction)> = HashSet::new();
    seen.insert((position, direction));
    loop {
        if let Some(next_step) = direction.mutate_coordinate(&position) {
            if seen.contains(&(next_step, direction)) {
                return true;
            }
            if !input.size.0.contains(&next_step.x) || !input.size.1.contains(&next_step.y) {
                break;
            }
            match input.walls.get(&next_step) {
                None => {
                    position = next_step;
                    seen.insert((position, direction));
                }
                Some(_) => {
                    direction = direction.next();
                }
            }
        } else {
            break;
        }
    }
    false
}
