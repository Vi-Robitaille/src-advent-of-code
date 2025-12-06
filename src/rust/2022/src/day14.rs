use aoc_runner_derive::{aoc, aoc_generator};
use std::{collections::HashSet, cmp::Ordering};
use glam::IVec2;
use tqdm::{Iter, Style};

type Point = IVec2;

const DEBUG: bool = false;

// 541,130 -> 541,122
fn range_generation(x: i32, y: i32) -> impl Iterator<Item=i32> {
    match x.cmp(&y) {
        Ordering::Less => {
            return x..=y;
        },
        Ordering::Greater => {
            return y..=x;
        },
        Ordering::Equal => {
            return 0..=0;
        },
    }
}

fn parse_to_point(s: &str) -> Point {
    let mut z: Vec<&str> = s.split(',').collect::<Vec<&str>>();
    let y = z.pop().unwrap().parse().unwrap();
    let x = z.pop().unwrap().parse().unwrap();
    if DEBUG { println!("Creating point x: {:?}, y: {:?}", x, y); }
    Point { x, y }
}


#[aoc_generator(day14)]
fn parse_input_day1(input: &str) -> HashSet<Point> {
    let mut hash: HashSet<Point> = HashSet::new();
    input
        .lines()
        .for_each(|line| {
            let mut positions: Vec<Point> = line.split(" -> ").map(|p| parse_to_point(p)).collect::<Vec<Point>>();
            let mut lhs: Point = positions.pop().unwrap();
            while let Some(rhs) = positions.pop() {
                match lhs - rhs {
                    Point {x: 0, y: _ } => {
                        if DEBUG { println!("X has not changed, using range [{}..={}]", lhs.y, rhs.y); }
                        range_generation(lhs.y, rhs.y).into_iter().for_each(|y| 
                            { hash.insert(Point {x: lhs.x, y}); }
                        );
                    },
                    Point {x: _, y: 0 } => {
                        if DEBUG { println!("Y has not changed, using range [{}..={}]", lhs.x, rhs.x); }
                        range_generation(lhs.x, rhs.x).into_iter().for_each(|x| {
                            hash.insert(Point {x, y: lhs.y}); }
                        );
                    },
                    _ => ()
                }
                lhs = rhs;
            }
        });
    hash
}

#[aoc(day14, part1)]
fn part_one(input: &HashSet<Point>) -> usize {
    if DEBUG { println!("Part 1 recieved {:?} elements", input.len()); }

    let mut local_hash = input.clone();

    let sand_origin = IVec2 { x: 500, y: 0 };
    let translations = [
        IVec2 { x: 1, y: 1},  // Down to the right
        IVec2 { x: -1, y: 1}, // Down to the left
        IVec2 { x: 0, y: 1},  // Down 
    ];

    for _ in (0..10000).tqdm().style(Style::Block) {
        let mut max_lifetime: i32 = 10000;
        let mut rest_flag: bool = false;
        let mut new_sand = sand_origin.clone();
        while max_lifetime > 0 && !rest_flag {
            max_lifetime -= 1;
            if DEBUG { println!("Sand is at {:?}", new_sand); }
           
            if let Some(new_position) = translations
                .clone()
                .into_iter()
                .filter(|f| {
                    !local_hash.contains(&(new_sand + *f))
                }).collect::<Vec<Point>>().pop() 
            {
                new_sand = new_sand + new_position;
                if DEBUG { println!(" - moving to x: {:?}, y: {:?}", new_sand.x, new_sand.y); }
            } else {
                // if we reach here this means we have no available spots left
                local_hash.insert(new_sand);
                rest_flag = true;
                if DEBUG { println!(" - adding sand at x: {:?}, y: {:?}", new_sand.x, new_sand.y); }
            }
        }
        if max_lifetime <= 0 { break; /* execution time has expired */ }
    }
    (&local_hash - input).len()
}

#[aoc(day14, part2)]
fn part_two(input: &HashSet<Point>) -> usize {
    if DEBUG { println!("Part 2 recieved {:?} elements", input.len()); }

    let mut local_hash = input.clone();

    let floor_position: i32 = 1 + local_hash.clone().into_iter().fold(i32::MIN, |a, b| a.max(b.y));

    let sand_origin = IVec2 { x: 500, y: 0 };
    let translations = [
        IVec2 { x: 1, y: 1},  // Down to the right
        IVec2 { x: -1, y: 1}, // Down to the left
        IVec2 { x: 0, y: 1},  // Down 
    ];

    for _ in (0..50000).tqdm().style(Style::Block) {
        let mut max_lifetime: i32 = 10000;
        let mut rest_flag: bool = false;
        let mut new_sand: Point = sand_origin.clone();
        while max_lifetime > 0 && !rest_flag {
            max_lifetime -= 1;
            if DEBUG { println!("Sand is at {:?}", new_sand); }
            
            if let Some(new_position) = translations
                .clone()
                .into_iter()
                .filter(|f| {
                    !local_hash.contains(&(new_sand + *f))
                }).collect::<Vec<Point>>().pop()
            {
                new_sand = new_sand + new_position;
                if DEBUG { println!(" - moving to x: {:?}, y: {:?}", new_sand.x, new_sand.y); }
            } else {
                // if we reach here this means we have no available spots left
                local_hash.insert(new_sand);
                rest_flag = true;
                if DEBUG { println!(" - adding sand at x: {:?}, y: {:?}", new_sand.x, new_sand.y); }
            }

            if new_sand.y >= floor_position { /* is it really advent of code if you dont bodge pt 2? */
                // if we reach here this means we have no available spots left
                local_hash.insert(new_sand);
                rest_flag = true;
            }
        }
        if max_lifetime <= 0 || new_sand == sand_origin { 
            break; /* execution time has expired or we found the answer */ 
        }
    }
    (&local_hash - input).len()
}
