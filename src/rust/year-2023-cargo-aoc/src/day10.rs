use aoc_runner_derive::{aoc, aoc_generator};

use geo::Intersects;
use geo::{coord, Coord, Line};
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
fn part_one(input: &Vec<Vec<char>>) -> usize {
    let start: Vec<(usize, usize)> = input
        .iter()
        .enumerate()
        .filter_map(|(i, e)| {
            let r = e
                .iter()
                .enumerate()
                .filter_map(|(j, x)| (*x == 'S').then(|| j))
                .collect_vec();
            (r.len() > 0).then(|| (r[0], i))
        })
        .collect_vec();
    let start_node = Node {
        x: start[0].0,
        y: start[0].1,
    };
    let grid_arc: Arc<Vec<Vec<char>>> = Arc::new(input.clone());

    let start_nodes_adjacent = get_adjacent(&start_node, &grid_arc).unwrap();
    let mut handles: Vec<JoinHandle<usize>> = vec![];

    for node in start_nodes_adjacent.iter() {
        let grid_arc: Arc<Vec<Vec<char>>> = Arc::clone(&grid_arc);
        let starting_node = start_node;
        let mut node = node.clone();
        let h = thread::spawn(move || {
            let mut path_len = 0;
            let mut previous_node = starting_node.clone();
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

#[aoc(day10, part2)]
fn part_two(input: &Vec<Vec<char>>) -> usize {
    // Same bullshit start finder, ya you can do it differently but
    //  its done this way. fight me.
    let start: Vec<(usize, usize)> = input
        .iter()
        .enumerate()
        .filter_map(|(i, e)| {
            let r = e
                .iter()
                .enumerate()
                .filter_map(|(j, x)| (*x == 'S').then(|| j))
                .collect_vec();
            (r.len() > 0).then(|| (r[0], i))
        })
        .collect_vec();

    //

    let start_node = Node {
        x: start[0].0,
        y: start[0].1,
    };
    let grid_arc: Arc<Vec<Vec<char>>> = Arc::new(input.clone());

    let start_nodes_adjacent = get_adjacent(&start_node, &grid_arc).unwrap();
    let mut handles: Vec<JoinHandle<Option<Vec<Line>>>> = vec![];

    for node in start_nodes_adjacent.iter() {
        // Values passed to the thread
        let grid_arc: Arc<Vec<Vec<char>>> = Arc::clone(&grid_arc);
        let starting_node = start_node;
        let mut node = node.clone();

        // The difference between part 1 and part 2 is that we return 
        // the path as a series of Lines we then use to test for intersections
        let h = thread::spawn(move || {
            let mut resulting_points: Vec<Line> = vec![];
            let mut previous_node = starting_node.clone();
            let mut previous_corner = starting_node.clone();

            while node != starting_node {
                if let Ok(next_node) = step(&node, &previous_node, &grid_arc) {
                    previous_node = node;
                    node = next_node;
                } else {
                    return None;
                }

                if node.is_corner(&grid_arc) {
                    resulting_points.push(Line::new(previous_corner.as_coord(), node.as_coord()));
                    previous_corner = node;
                }
            }

            resulting_points.push(Line::new(
                previous_corner.as_coord(),
                starting_node.as_coord(),
            ));
            Some(resulting_points)
        });
        handles.push(h);
    }


    let mut contained: usize = 0;
    for j in handles {
        let r = j.join();
        if let Ok(Some(path)) = r {
            // To test if the point is inside or outside the path we evaluate 
            // the number of collisions with paths
            // We may need to subtract the number of contained Lines
            let ground_points = get_all_ground_nodes(&grid_arc);
            
            contained = ground_points.iter()
                .filter(|&&p| {
                    // Add a slight offset to the endpoint since detecting overlaps is annoying
                    // 1.7319 is the ratio of a triangle to have one angle at 30 deg
                    let test_line = Line::new(p, coord! { x: -1.0, y: p.y - ((p.x + 1.0) / 1.7319)});
                    let intersections = path.iter().filter(|l| {
                            l.intersects(&test_line)
                        })
                        .count();
                    intersections > 0 && intersections % 2 == 1
                })
                .map(|x| println!("({:?},{:?})", x.x, x.y))
                .count();
            path.iter()
                .for_each(|x| println!("((1-t){} + t{}, (1-t){} + t{})", x.start.x, x.end.x, x.start.y, x.end.y));
            println!("-------------")
        }
    }
    contained
}

fn get_adjacent(n: &Node, grid: &Arc<Vec<Vec<char>>>) -> Result<Vec<Node>, ()> {
    let x_bounds: Range<usize> = 0..grid[0].len();
    let y_bounds: Range<usize> = 0..grid.len();
    let nodes = [
        Node { x: n.x, y: n.y - 1 }, // North
        Node { x: n.x + 1, y: n.y }, // East
        Node { x: n.x, y: n.y + 1 }, // South
        Node { x: n.x - 1, y: n.y }, // West
    ];

    match PipeTypes::from((grid[n.y][n.x], nodes)) {
        PipeTypes::Ground => return Err(()),
        PipeTypes::Start(a, b, c, d) => {
            let nodes = vec![a, b, c, d]
                .iter()
                .filter(|v| v.is_valid(&x_bounds, &y_bounds))
                .map(|&v| v)
                .collect_vec();
            Ok(nodes)
        }

        PipeTypes::Vertical(a, b)
        | PipeTypes::Horizontal(a, b)
        | PipeTypes::NENinety(a, b)
        | PipeTypes::NWNinety(a, b)
        | PipeTypes::SWNinety(a, b)
        | PipeTypes::SENinety(a, b) => {
            let nodes = vec![a, b]
                .iter()
                .filter(|v| v.is_valid(&x_bounds, &y_bounds))
                .map(|&v| v)
                .collect_vec();
            Ok(nodes)
        }
    }
}

enum PipeTypes {
    Vertical(Node, Node),
    Horizontal(Node, Node),
    NENinety(Node, Node),
    NWNinety(Node, Node),
    SWNinety(Node, Node),
    SENinety(Node, Node),
    Ground,
    Start(Node, Node, Node, Node),
}

impl From<(char, [Node; 4])> for PipeTypes {
    fn from(value: (char, [Node; 4])) -> Self {
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
    fn is_valid(self, x_bounds: &Range<usize>, y_bounds: &Range<usize>) -> bool {
        x_bounds.contains(&self.x) && y_bounds.contains(&self.y)
    }
    fn as_coord(self) -> Coord<f64> {
        coord! {
            x: self.x as f64,
            y: self.y as f64,
        }
    }
    fn is_corner(&self, grid: &Arc<Vec<Vec<char>>>) -> bool {
        match grid[self.y][self.x] {
            'L' | 'J' | '7' | 'F' => true,
            _ => false,
        }
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
            _ => return false,
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
    if let Ok(nodes) = get_adjacent(current_node, grid) {
        let non_prev_nodes = nodes.iter().filter(|&x| x != previous_node).collect_vec();
        if non_prev_nodes.len() == 1 {
            return Ok(non_prev_nodes[0].clone());
        }
    }
    Err(())
}

fn get_all_ground_nodes(grid: &Arc<Vec<Vec<char>>>) -> Vec<Coord<f64>> {
    let mut res: Vec<Coord<f64>> = vec![];
    for (y, i) in grid.iter().enumerate() {
        for (x, e) in i.iter().enumerate() {
            if e == &'.' {
                res.push(Node { x, y }.as_coord());
            }
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
