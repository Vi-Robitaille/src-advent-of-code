use aoc_runner_derive::{aoc, aoc_generator};

use geo::Contains;
use geo::{coord, Coord, LineString, Polygon};
use itertools::Itertools;
use std::ops::Range;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

const N: usize = 0;
const E: usize = 1;
const S: usize = 2;
const W: usize = 3;

#[aoc_generator(day10)]
fn parse_input_day1(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|x| x.chars().collect_vec()).collect_vec()
}

#[aoc(day10, part1)]
fn part_one(input: &[Vec<char>]) -> usize {
    let start: Vec<(usize, usize)> = input
        .iter()
        .enumerate()
        .filter_map(|(i, e)| {
            let r = e
                .iter()
                .enumerate()
                .filter_map(|(j, x)| (*x == 'S').then_some(j))
                .collect_vec();
            (!r.is_empty()).then(|| (r[0], i))
        })
        .collect_vec();
    let start_node = Node {
        x: start[0].0,
        y: start[0].1,
    };
    let grid_arc: Arc<Vec<Vec<char>>> = Arc::new(input.to_owned());

    let start_nodes_adjacent = get_adjacent(&start_node, &grid_arc);
    let mut handles: Vec<JoinHandle<usize>> = vec![];

    for node in start_nodes_adjacent.iter() {
        let grid_arc: Arc<Vec<Vec<char>>> = Arc::clone(&grid_arc);
        let starting_node = start_node;
        let mut node = *node;
        let h = thread::spawn(move || {
            let mut path_len = 0;
            let mut previous_node = starting_node;
            while node != starting_node {
                let next_node = step(&node, &previous_node, &grid_arc);
                if let Ok(next_node) = next_node {
                    previous_node = node;
                    node = next_node;
                } else {
                    path_len = 0;
                    return path_len;
                }
                path_len += 1;
            }
            (path_len + 1) / 2
        });
        handles.push(h);
    }

    let mut res: Vec<usize> = vec![];
    for jh in handles {
        let r = jh.join();
        res.push(r.map_or(0, |v| v));
    }
    res.sort();
    println!("{:?}", res);
    res.iter().last().unwrap().to_owned()
}

///
/// You know when you've sunk a lot of time into a solution
/// and you dont want to abandon it
/// but it gets really gross
/// and you keep pushing to make it work
/// but its just not a great solution?
///
/// This is that
///
/// im not cleaning it.
///
#[aoc(day10, part2)]
fn part_two(input: &[Vec<char>]) -> usize {
    // Same bullshit start finder, ya you can do it differently but
    //  its done this way. fight me.
    let start: Vec<(usize, usize)> = input
        .iter()
        .enumerate()
        .filter_map(|(i, e)| {
            let r = e
                .iter()
                .enumerate()
                .filter_map(|(j, x)| (*x == 'S').then_some(j))
                .collect_vec();
            (!r.is_empty()).then(|| (r[0], i))
        })
        .collect_vec();

    //

    let start_node = Node {
        x: start[0].0,
        y: start[0].1,
    };
    let grid_arc: Arc<Vec<Vec<char>>> = Arc::new(input.to_owned());

    let start_nodes_adjacent = get_adjacent(&start_node, &grid_arc);
    let mut handles: Vec<JoinHandle<Option<LineString<f64>>>> = vec![];

    for node in start_nodes_adjacent.iter() {
        // Values passed to the thread
        let grid_arc: Arc<Vec<Vec<char>>> = Arc::clone(&grid_arc);
        let starting_node = start_node;
        let mut node = *node;

        // The difference between part 1 and part 2 is that we return
        // the path as a series of Lines we then use to test for intersections
        let h = thread::spawn(move || {
            let mut resulting_points: Vec<Coord> = vec![];
            let mut previous_node = starting_node;

            while node != starting_node {
                if let Ok(next_node) = step(&node, &previous_node, &grid_arc) {
                    previous_node = node;
                    node = next_node;
                } else {
                    return None;
                }
                resulting_points.push(node.as_coord());
            }

            resulting_points.push(starting_node.as_coord());
            Some(LineString::new(resulting_points))
        });
        handles.push(h);
    }

    solve_via_polygon(handles, &grid_arc)
}

type YuckLineString = Vec<JoinHandle<Option<LineString<f64>>>>;
type GridYuck = Arc<Vec<Vec<char>>>;
///
/// The print statements in this output a desmos formatted paste
/// so you can toss it into a desmos graph to see how dumb i am
///
#[allow(unused)]
fn solve_via_polygon(handles: YuckLineString, grid: &GridYuck) -> usize {
    let mut contained: usize = 0;
    for j in handles {
        let r = j.join();
        if let Ok(Some(path)) = r {
            // To test if the point is inside or outside the path we evaluate
            // the number of collisions with paths
            // We may need to subtract the number of contained Lines
            let ground_points = get_all_nodes(grid);

            let poly = Polygon::new(path, vec![]);

            contained = ground_points.iter().filter(|&&p| poly.contains(&p)).count();
        }
    }
    contained
}

fn get_adjacent(n: &Node, grid: &Arc<Vec<Vec<char>>>) -> Vec<Node> {
    let x_bounds: Range<usize> = 0..grid[0].len();
    let y_bounds: Range<usize> = 0..grid.len();
    let nodes = [
        n.get_north(&y_bounds), // North
        n.get_east(&x_bounds),  // East
        n.get_south(&y_bounds), // South
        n.get_west(&x_bounds),  // West
    ];

    match PipeTypes::from((grid[n.y][n.x], nodes)) {
        PipeTypes::Ground => vec![],
        PipeTypes::Start(a, b, c, d) => {
            let nodes = [a, b, c, d].iter().flatten().cloned().collect_vec();
            nodes
        }

        PipeTypes::Vertical(a, b)
        | PipeTypes::Horizontal(a, b)
        | PipeTypes::NENinety(a, b)
        | PipeTypes::NWNinety(a, b)
        | PipeTypes::SWNinety(a, b)
        | PipeTypes::SENinety(a, b) => {
            let nodes = [a, b].iter().flatten().cloned().collect_vec();
            nodes
        }
    }
}

enum PipeTypes {
    Vertical(Option<Node>, Option<Node>),
    Horizontal(Option<Node>, Option<Node>),
    NENinety(Option<Node>, Option<Node>),
    NWNinety(Option<Node>, Option<Node>),
    SWNinety(Option<Node>, Option<Node>),
    SENinety(Option<Node>, Option<Node>),
    Ground,
    Start(Option<Node>, Option<Node>, Option<Node>, Option<Node>),
}

impl From<(char, [Option<Node>; 4])> for PipeTypes {
    fn from(value: (char, [Option<Node>; 4])) -> Self {
        match value.0 {
            '|' => PipeTypes::Vertical(value.1[N], value.1[S]),
            '-' => PipeTypes::Horizontal(value.1[E], value.1[W]),
            'L' => PipeTypes::NENinety(value.1[N], value.1[E]),
            'J' => PipeTypes::NWNinety(value.1[N], value.1[W]),
            '7' => PipeTypes::SWNinety(value.1[W], value.1[S]),
            'F' => PipeTypes::SENinety(value.1[E], value.1[S]),
            '.' => PipeTypes::Ground,
            'S' => PipeTypes::Start(value.1[N], value.1[E], value.1[S], value.1[W]),
            e => panic!("Invalid value {e:?}"),
        }
    }
}

// [Node; 4]
// [North, East, South, West]
//           x      y
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    x: usize,
    y: usize,
}

impl Node {
    fn as_coord(self) -> Coord<f64> {
        coord! {
            x: self.x as f64,
            y: self.y as f64,
        }
    }
    #[allow(dead_code)]
    fn is_corner(&self, grid: &Arc<Vec<Vec<char>>>) -> bool {
        matches!(grid[self.y][self.x], 'L' | 'J' | '7' | 'F')
    }
    fn get_north(&self, _y_bounds: &Range<usize>) -> Option<Self> {
        if let Some(y) = self.y.checked_sub(1) {
            return Some(Node { x: self.x, y });
        }
        None
    }
    fn get_east(&self, x_bounds: &Range<usize>) -> Option<Self> {
        let x = self.x + 1;
        if x_bounds.contains(&x) {
            return Some(Node { x, y: self.y });
        }
        None
    }
    fn get_south(&self, y_bounds: &Range<usize>) -> Option<Self> {
        let y = self.y + 1;
        if y_bounds.contains(&y) {
            return Some(Node { x: self.x, y });
        }
        None
    }
    fn get_west(&self, _x_bounds: &Range<usize>) -> Option<Self> {
        if let Some(x) = self.x.checked_sub(1) {
            return Some(Node { x, y: self.y });
        }
        None
    }
}

#[allow(unused)]
struct Span {
    start: Node,
    end: Node,
}

/// in all of these functions lhs is assumed to be the entire
/// width or height of the area being evaluated
impl Span {
    ///
    /// This tests if a given span is fully within this span
    ///
    /// A: (1, 1) -> (1, 7)
    /// B: (0, 4) -> (10, 4)
    ///
    /// C: (0, 9) -> (10, 0)
    /// D: (6, 9) -> (9, 9)
    ///
    /// A -> B : False
    /// C -> D : True
    ///
    /// ...........
    /// .B-------7.
    /// .BF-----7|.
    /// .B|.....||.
    /// AAAAAAAAAAA
    /// .BL-7.F-J|.
    /// .B..|.|..|.
    /// CBCCCCDDDDC
    /// ...........
    ///
    #[allow(unused)]
    fn contains(&self, rhs: &Self) -> bool {
        // These should be zero if they are colinear
        let same_start_x = self.start.x.checked_sub(rhs.start.x);
        let same_end_x = self.end.x.checked_sub(rhs.end.x);
        let same_start_y = self.start.y.checked_sub(rhs.start.y);
        let same_end_y = self.end.y.checked_sub(rhs.end.y);

        // Evaluate the differences between the lines
        match (same_start_x, same_start_y, same_end_x, same_end_y) {
            // both X are zero, a vertical line and they are colinear
            (Some(0), _, Some(0), _) => {
                self.start.y <= rhs.start.y
                    && self.start.y <= rhs.end.y
                    && self.end.y >= rhs.start.y
                    && self.end.y >= rhs.end.y
            }
            // both Y are zero, a horizontal line and they are colinear
            (_, Some(0), _, Some(0)) => {
                self.start.x <= rhs.start.x
                    && self.start.x <= rhs.end.x
                    && self.end.x >= rhs.start.x
                    && self.end.x >= rhs.end.x
            }
            _ => false,
        }
    }

    /// Tests if two spans intersect
    #[allow(unused)]
    fn intersection(&self, rhs: &Self) -> bool {
        if self.contains(rhs) {
            return false;
        }

        let axy = (self.start.x..self.end.x, self.start.y..self.end.y);
        let bxy = (rhs.start.x..rhs.end.x, rhs.start.y..rhs.end.y);

        // its at this point that i'm just going to abandon this
        // and use rust
        false
    }
}

/// From any given spot there will be one valid direction to
/// step towards if we know where we're coming from.
fn step(current_node: &Node, previous_node: &Node, grid: &Arc<Vec<Vec<char>>>) -> Result<Node, ()> {
    let nodes = get_adjacent(current_node, grid);
    if nodes.is_empty() {
        return Err(());
    }

    let non_prev_nodes = nodes.iter().filter(|&x| x != previous_node).collect_vec();
    if non_prev_nodes.len() == 1 {
        return Ok(*non_prev_nodes[0]);
    }
    Err(())
}

fn get_all_nodes(grid: &Arc<Vec<Vec<char>>>) -> Vec<Coord<f64>> {
    let mut res: Vec<Coord<f64>> = vec![];
    for (y, i) in grid.iter().enumerate() {
        for (x, _e) in i.iter().enumerate() {
            res.push(Node { x, y }.as_coord());
        }
    }
    res
}

// ...........
// .S-------7.
// .|F-----7|.
// .||.....||.
// .||.....||.
// .|L-7.F-J|.
// .|..|.|..|.
// .L--J.L--J.
// ...........

// We can use point in polygon testing
// https://i.stack.imgur.com/05eHj.png

// if the intersection count is odd then we know its in the count
// to do this we need to test if the line evaluating its position is coplanar with
// a polygon face or evaluate the polygon from a non orthogonal direction

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../input/2023/day10.txt");
    const TEST_INPUT_1: &'static str = include_str!("../input/2023/day10-test.txt");
    const TEST_INPUT_2: &'static str = include_str!("../input/2023/day10-test2.txt");
    const TEST_INPUT_3: &'static str = include_str!("../input/2023/day10-test3.txt");
    const TEST_INPUT_4: &'static str = include_str!("../input/2023/day10-test4.txt");
    const TEST_INPUT_5: &'static str = include_str!("../input/2023/day10-test5.txt");
    const TEST_INPUT_6: &'static str = include_str!("../input/2023/day10-test6.txt");

    #[test]
    fn part_1_test_1() {
        let inp = parse_input_day1(TEST_INPUT_1);
        assert_eq!(part_one(&inp), 4)
    }
    #[test]
    fn part_1_test_2() {
        let inp = parse_input_day1(TEST_INPUT_2);
        assert_eq!(part_one(&inp), 8)
    }

    #[test]
    fn part_1_input() {
        let inp = parse_input_day1(INPUT);
        assert_eq!(part_one(&inp), 6979)
    }

    #[test]
    fn part_2_sample_3() {
        let inp = parse_input_day1(TEST_INPUT_3);
        assert_eq!(part_two(&inp), 4)
    }

    #[test]
    fn part_2_sample_4() {
        let inp = parse_input_day1(TEST_INPUT_4);
        assert_eq!(part_two(&inp), 4)
    }

    #[test]
    fn part_2_sample_5() {
        let inp = parse_input_day1(TEST_INPUT_5);
        assert_eq!(part_two(&inp), 8)
    }

    #[test]
    fn part_2_sample_6() {
        let inp = parse_input_day1(TEST_INPUT_6);
        assert_eq!(part_two(&inp), 10)
    }

    #[test]
    fn part_2_input() {
        let inp = parse_input_day1(INPUT);
        assert_eq!(part_two(&inp), 443)
    }
}
