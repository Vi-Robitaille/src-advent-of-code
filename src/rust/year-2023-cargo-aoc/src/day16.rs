use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};
use itertools::Itertools;
use itertools::chain;
use grid::*;

#[aoc_generator(day16)]
fn parse_input(input: &str) -> Grid<NodeType> {
    let g = input.lines().flat_map(|s| s.chars().map(|c| NodeType::from(c)).collect_vec()).collect_vec();
    let cols = (g.len() as f64).sqrt() as usize;
    Grid::from_vec(g, cols)
}

#[aoc(day16, part1)]
fn part_one(input: &Grid<NodeType>) -> usize {
    let visited_cells: Arc<Mutex<HashSet<Coordinate>>> = Arc::new(Mutex::new(HashSet::new()));
    let visited_pairs: Arc<Mutex<HashSet<(Coordinate, Coordinate)>>> = Arc::new(Mutex::new(HashSet::new()));
    let grid_arc: Arc<Grid<NodeType>> = Arc::new(input.clone());
    let position: Coordinate = (0, 0);
    let heading: Heading = Heading::E;
    let handle = spawn_thread(&position, &heading, &visited_cells, &visited_pairs, &grid_arc);
    let _ = handle.join();
    let res = visited_cells.lock().unwrap().iter().count();
    res
}

#[aoc(day16, part2)]
fn part_two(input: &Grid<NodeType>) -> usize {
    // Just loop over all the border cells and eval the bigliest one
    let mut res = 0;
    // its probably square?
    let grid_size = input.size().0;
    let a = (0..grid_size).map(|n| ((n as isize, 0 as isize), Heading::S));
    let b = (0..grid_size).map(|n| ((0 as isize, n as isize), Heading::E));
    let c = (0..grid_size).map(|n| ((n as isize, grid_size as isize), Heading::N));
    let d = (0..grid_size).map(|n| ((grid_size as isize, n as isize), Heading::W));


    for (position, heading) in chain!(a,b,c,d) {
        let visited_cells: Arc<Mutex<HashSet<Coordinate>>> = Arc::new(Mutex::new(HashSet::new()));
        let visited_pairs: Arc<Mutex<HashSet<(Coordinate, Coordinate)>>> = Arc::new(Mutex::new(HashSet::new()));
        let grid_arc: Arc<Grid<NodeType>> = Arc::new(input.clone());
        // let position: Coordinate = (0, 0);
        // let heading: Heading = Heading::E;
        let handle = spawn_thread(&position, &heading, &visited_cells, &visited_pairs, &grid_arc);
        let _ = handle.join();
        let r = visited_cells.lock().unwrap().iter().count();
        if r > res { res = r };
    }
    res
}


fn follow_path(c: Coordinate, h: Heading, hm: Arc<Mutex<HashSet<Coordinate>>>, visited_pairs: Arc<Mutex<HashSet<(Coordinate, Coordinate)>>>, grid_arc: Arc<Grid<NodeType>>) {
    // println!("Thread starting with {:?} and heading {:?}", c, h);
    // let ((mut x, mut y), mut h) = (c, h);
    let mut c = c;
    let mut h = h;
    // let mut visited = vec![c];
    let mut thread_handles: Vec<JoinHandle<()>> = vec![];
    while let Some(next_node) = get_node(c.0, c.1, &grid_arc) {
        // println!("Thread evaluating {:?}", c);
        if let Ok(mut mut_guard) = hm.lock() {
            // println!("Adding {:?} to visited", c);
            let _ = (*mut_guard).insert(c);
        }
        if let Ok(mut mut_guard) = visited_pairs.lock() {
            // println!("Adding {:?} to visited", c);
            let seen = (*mut_guard).insert((c, h.apply(c)));
            if !seen {
                break;
            }
        }
        match next_node.process_node(c, h) {
            BeamCharacteristic::Normal(nh) => { 
                // println!("Moving from {:?} -> {:?}:{:?}", c, h.apply(c), h);
                h = nh;
                c = h.apply(c);
             },
            BeamCharacteristic::Split(a) => {
                // Puke
                let old_c = c;
                h = a[0];
                c = h.apply(c);
                // println!("Found a split [{:?} -> {:?}:{:?}],[{:?} -> {:?}:{:?}]", old_c, c, h, old_c, a[1].apply(old_c), a[1]);
                let handle = spawn_thread(&a[1].apply(old_c), &a[1], &hm, &visited_pairs, &grid_arc);
                thread_handles.push(handle);
            }
        }
    }
    // println!("Joining {} threads", thread_handles.len());
    for t in thread_handles {
        let _ = t.join();
    }
}


fn spawn_thread(thread_coord: &Coordinate, thread_heading: &Heading, thread_hm: &Arc<Mutex<HashSet<Coordinate>>>, thread_pairs: &Arc<Mutex<HashSet<(Coordinate, Coordinate)>>>, thread_grid: &Arc<Grid<NodeType>>) -> JoinHandle<()> {
    let thread_coord = thread_coord.clone();
    let thread_heading = thread_heading.clone();
    let thread_hm = Arc::clone(thread_hm);
    let thread_pairs = Arc::clone(thread_pairs);
    let thread_grid = Arc::clone(thread_grid);
    thread::spawn(move || follow_path(thread_coord, thread_heading, thread_hm, thread_pairs, thread_grid))
}

/// ReflectRight would be as if you are shining a flashlight 
/// at it and the light would reflect onto a wall to the right
/// of you
/// 
/// . = Ground
/// O = Origin
/// X = Light Path
/// \ = Reflect Right Mirror
/// 
/// .....
/// .....
/// OX\..
/// ..X..
/// ..X..
/// 
#[derive(Debug, Clone, Copy)]
enum NodeType {
    Empty,
    SplitVertical,
    SplitHorizontal,
    ReflectRight,
    ReflectLeft
}

impl From<char> for NodeType {
    fn from(value: char) -> Self {
        match value {
            '.' => NodeType::Empty,
            '-' => NodeType::SplitHorizontal,
            '|' => NodeType::SplitVertical,
            '/' => NodeType::ReflectLeft,
            '\\' => NodeType::ReflectRight,
            _ => panic!()
        }
    }
}

impl NodeType {
    fn process_node(&self, c: Coordinate, h: Heading) -> BeamCharacteristic {
        // let c = h.apply(c);
        // println!("Moving from {:?} to {:?}", v, c);
        match (h, *self) {
            (_, Self::Empty) => BeamCharacteristic::Normal(h),
            (Heading::E, Self::SplitHorizontal) | (Heading::W, Self::SplitHorizontal) => BeamCharacteristic::Normal(h),
            (Heading::N, Self::SplitHorizontal) | (Heading::S, Self::SplitHorizontal) => BeamCharacteristic::Split([
                Heading::E, Heading::W
            ]),
            (Heading::N, Self::SplitVertical) | (Heading::S, Self::SplitVertical) => BeamCharacteristic::Normal(h),
            (Heading::E, Self::SplitVertical) | (Heading::W, Self::SplitVertical) => BeamCharacteristic::Split([
                Heading::N, Heading::S
            ]),
            (Heading::N, Self::ReflectLeft) => BeamCharacteristic::Normal(Heading::E),
            (Heading::E, Self::ReflectLeft) => BeamCharacteristic::Normal(Heading::N),
            (Heading::S, Self::ReflectLeft) => BeamCharacteristic::Normal(Heading::W),
            (Heading::W, Self::ReflectLeft) => BeamCharacteristic::Normal(Heading::S),

            (Heading::N, Self::ReflectRight) => BeamCharacteristic::Normal(Heading::W),
            (Heading::E, Self::ReflectRight) => BeamCharacteristic::Normal(Heading::S),
            (Heading::S, Self::ReflectRight) => BeamCharacteristic::Normal(Heading::E),
            (Heading::W, Self::ReflectRight) => BeamCharacteristic::Normal(Heading::N),
        }
    }
}


/// Normally a thread will continue along its path
///  when reflected it will not change
///   when split we will spawn a thread.
enum BeamCharacteristic {
    Normal(Heading),
    Split([Heading; 2])
}

type Coordinate = (isize, isize);

#[derive(Debug, Clone, Copy)]
enum Heading {
    N, S, E, W
}

impl Into<(isize, isize)> for Heading {
    fn into(self) -> (isize, isize) {
        match self {
            Self::N => (0, -1),
            Self::E => (1,  0),
            Self::S => (0,  1),
            Self::W => (-1, 0),
        }
    }
}

impl Heading {
    fn apply(&self, c: Coordinate) -> Coordinate {
        let h: (isize, isize) = (*self).into();
        let c: Coordinate = (c.0 + h.0, c.1 + h.1);
        (c.0, c.1)
    }
}

/// The inverse x,y of grid is gonna hurt my braaaaaaaaaaaaain
fn get_node<'a>(x: isize, y: isize, grid: &'a Grid<NodeType>) -> Option<&'a NodeType> {
    // println!("Getting {x},{y}");
    match (y.try_into(), x.try_into()) {
        (Ok(y), Ok(x)) => grid.get(y, x),
        _ => None
    }
}
