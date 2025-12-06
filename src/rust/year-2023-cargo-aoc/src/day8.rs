use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use lazy_static::lazy_static;
use prime_factorization::Factorization;
use regex::Regex;
use std::collections::HashMap;

// Try using nom later
lazy_static! {
    static ref NODE_REGEX: Regex = Regex::new(r"[1-9A-Z]{3}").unwrap();
}

#[aoc_generator(day8)]
fn parse_input_day1(input: &str) -> (String, HashMap<String, Node>) {
    let mut inp_iter = input.lines();

    let key = inp_iter.next().unwrap().to_owned();
    let _ = inp_iter.next();

    let node_hm: HashMap<String, Node> = inp_iter
        .map(|line| {
            let mut node_iter = NODE_REGEX.find_iter(line);
            let node_key = node_iter.next().unwrap().as_str().to_string();
            let node_l = node_iter.next().unwrap().as_str().to_string();
            let node_r = node_iter.next().unwrap().as_str().to_string();
            (
                node_key,
                Node {
                    l: node_l,
                    r: node_r,
                },
            )
        })
        .collect();

    (key, node_hm)
}

#[aoc(day8, part1)]
fn part_one(input: &(String, HashMap<String, Node>)) -> usize {
    let starting_node = "AAA";
    let mut current_node = starting_node;

    let mut steps = input.0.chars().cycle();
    let mut iterations: usize = 0;

    while current_node != "ZZZ" {
        let node = if let Some(node) = input.1.get(current_node) {
            node
        } else {
            panic!();
        };
        match steps.next().unwrap() {
            'R' => {
                current_node = &node.r;
            }
            'L' => {
                current_node = &node.l;
            }
            _ => panic!("Bad character."),
        }
        iterations += 1;
    }

    iterations
}

#[aoc(day8, part2)]
fn part_two(input: &(String, HashMap<String, Node>)) -> usize {
    let starting_nodes: Vec<String> = input
        .1
        .keys()
        .filter(|&a| a.ends_with('A'))
        .cloned()
        .collect();

    // hashmap of <prime, times this prime has been seen>
    let mut result: HashMap<_, _> = HashMap::new();
    for s in starting_nodes.iter() {
        let x = find_loops(s, input.0.clone(), &input.1);
        let f = Factorization::run(x as u32).factors;

        for (&k, v) in f.iter().counts() {
            result
                .entry(k)
                .and_modify(|val| {
                    if v > *val {
                        *val = v
                    }
                })
                .or_insert(v);
        }
    }
    println!("{:?}", result);

    result
        .keys()
        .map(|k| *k as usize)
        .reduce(|a, b| a * b)
        .unwrap()
}

fn find_loops(starting_node: &str, steps: String, hm: &HashMap<String, Node>) -> usize {
    let mut start_to_z_term = 0;

    let mut steps = steps.chars().cycle();
    let mut current_node = starting_node;

    while !current_node.ends_with('Z') {
        let node = if let Some(node) = hm.get(current_node) {
            node
        } else {
            panic!();
        };
        match steps.next().unwrap() {
            'R' => {
                current_node = &node.r;
            }
            'L' => {
                current_node = &node.l;
            }
            _ => panic!("Bad character."),
        }
        start_to_z_term += 1;
    }

    start_to_z_term
}

struct Node {
    r: String,
    l: String,
}
