use aoc_runner_derive::{aoc, aoc_generator};

use grid::*;
use itertools::Itertools;
use lazy_static::lazy_static;
use Heading::*;

lazy_static! {
    static ref SUCCESSOR_PATHS: Vec<Vec<Heading>> = vec![
        vec![UP, LEFT], vec![UP, RIGHT],
        vec![DOWN, LEFT], vec![DOWN, RIGHT],
        vec![UP, UP, LEFT], vec![UP, UP, RIGHT], 
        vec![DOWN, DOWN, LEFT], vec![DOWN, DOWN, RIGHT], 
        vec![UP, UP, LEFT], vec![UP, UP, RIGHT], 
        vec![DOWN, DOWN, LEFT], vec![DOWN, DOWN, RIGHT], 
        vec![UP, UP, UP, LEFT], vec![UP, UP, UP, RIGHT], 
        vec![DOWN, DOWN, DOWN, LEFT], vec![DOWN, DOWN, DOWN, RIGHT], 
        vec![RIGHT, UP], vec![RIGHT, DOWN],
        vec![LEFT, UP], vec![LEFT, DOWN],
        vec![RIGHT, RIGHT, UP], vec![RIGHT, RIGHT, DOWN],
        vec![LEFT, LEFT, UP], vec![LEFT, LEFT, DOWN],
        vec![RIGHT, RIGHT, RIGHT, UP], vec![RIGHT, RIGHT, RIGHT, DOWN],
        vec![LEFT, LEFT, LEFT, UP], vec![LEFT, LEFT, LEFT, DOWN],
    ];
}

static mut GOAL: Point = Point { x: 0, y: 0 };
/// Fight me.
unsafe fn set_goal(p: Point) {
    GOAL = p;
}

#[aoc_generator(day17)]
fn parse_input(input: &str) -> Vec<()> {
    vec![()]
}

#[aoc(day17, part1)]
fn part_one(input: &Vec<()>) -> usize {
    1
}

#[aoc(day17, part2)]
fn part_two(input: &Vec<()>) -> usize {
    1
}


enum Heading {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    fn apply_heading(&self, h: &Heading) -> Option<Self> {
        match *h {
            Heading::UP    => self.sub(0, 1),
            Heading::RIGHT => self.add(1, 0),
            Heading::DOWN  => self.add(0, 1),
            Heading::LEFT  => self.sub(1, 0)
        }
    }

    fn add(&self, x: usize, y: usize) -> Option<Self> {
        match (self.x.checked_add(x), self.y.checked_add(y)) {
            (Some(a), Some(b)) => Some(Point::new(a, b)),
            _ => None
        }
    }
    fn sub(&self, x: usize, y: usize) -> Option<Self> {
        match (self.x.checked_sub(x), self.y.checked_sub(y)) {
            (Some(a), Some(b)) => Some(Point::new(a, b)),
            _ => None
        }
    }
}

struct Town {
    positions: Grid<u8>,
}

impl Town {

    ///
    /// This should yield the following points
    /// 
    /// ```
    ///  .........
    ///  ...X.X...
    ///  ...X.X...
    ///  .XX2.2XX.
    ///  ....O....
    ///  .XX2.2XX.
    ///  ...X.X...
    ///  ...X.X...
    ///  .........
    /// ```
    /// 
    /// For each cell labeled 2 we should return the cost via
    /// each neighbor as there are two paths there
    /// 
    fn successors(&self, origin: Point) -> Vec<(Point, usize)> {
        SUCCESSOR_PATHS.iter()
            // Apply all the headings per path to the origin returning a list of 
            //  valid nodes for calculating the cost to that node
            .filter_map(|path| {
                // Apply the heading
                let nodes = path.iter()
                    .map(|heading| origin.apply_heading(heading))
                    .map(|p| {
                        if let Some(p) = p {
                            if let Some(g) = self.positions.get(p.y, p.x) {
                                return Some((p, *g as usize));
                            } 
                        }
                        None
                    }).collect_vec();
                if nodes.iter().all(|&p| p.is_some()) {
                    let dest_p = nodes.last().unwrap().unwrap().0;
                    let cost = nodes.iter().map(|a| a.unwrap().1).fold(0, |a, b| a + b);
                    Some((dest_p, cost))
                } else {
                    None
                }
            }).collect_vec()
    }    
}