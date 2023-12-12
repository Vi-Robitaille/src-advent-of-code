use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;
use std::ops::Range;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use geo::{LineString, Polygon, coord, Coord};
use geo::{ConcaveHull, Contains};

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
    let mut handles: Vec<JoinHandle<Option<Vec<Coord<f32>>>>> = vec![];

    for node in start_nodes_adjacent.iter() {
        // Values passed to the thread
        let grid_arc: Arc<Vec<Vec<char>>> = Arc::clone(&grid_arc);
        let starting_node = start_node;
        let mut node = node.clone();

        let h = thread::spawn(move || {
            let mut resulting_points: Vec<Coord<f32>> = vec![start_node.as_coord()];
            let mut previous_node = starting_node.clone();
            while node != starting_node {
                let next_node = step(&node, &previous_node, &grid_arc);
                if let Ok(next_node) = next_node {
                    previous_node = node;
                    node = next_node;
                } else {
                    return None;
                }
                
                if node.is_corner(&grid_arc) {
                    resulting_points.push(node.as_coord());
                }
            }
            
            resulting_points.push(start_node.as_coord());
            Some(resulting_points)
        });
        handles.push(h);
    }

    for j in handles {
        let r = j.join();
        if let Ok(Some(path)) = r {
            let poly = Polygon::new(LineString::from(path), vec![]).concave_hull(0.0001);

            let coords = get_all_ground_nodes_as_coords(&grid_arc);
            let s = coords
                .iter()
                .filter(|&c| poly.contains(c))
                .count();
            println!("this loop contains {s:?} elements");
            println!("{:?}", coords.len())
        }
    }
    0
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

    fn as_coord(self) -> Coord<f32> {
        coord! {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
    fn is_corner(&self, grid: &Arc<Vec<Vec<char>>>) -> bool {
        match grid[self.y][self.x] { 
            'L' | 'J' | '7' | 'F' => true,
            _ => false
        }
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

fn get_all_ground_nodes_as_coords(grid: &Arc<Vec<Vec<char>>>) -> Vec<Coord<f32>> {
    let mut res: Vec<Coord<f32>> = vec![];
    for (x, i) in grid.iter().enumerate() {
        for (y, e) in i.iter().enumerate() {
            if e == &'.' {
                res.push(coord! {x: x as f32, y: y as f32 });
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
