use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;

use cached::proc_macro::cached;

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Vec<(Vec<usize>, Vec<usize>)> {
    input
        .lines()
        .map(|x| x.split(' '))
        .map(|mut x| {
            let states = new_state_string(
                x.next().unwrap().chars().collect_vec()
            );
            let keys = x
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect_vec();
            (states, keys)
        })
        .collect_vec()
}

#[aoc(day12, part1)]
fn part_one(input: &Vec<(Vec<usize>, Vec<usize>)>) -> usize {
    let mut sum = 0;
    for (s, k) in input {
        sum += count(s.to_owned(), k.to_owned());
        println!("{sum}");
    }
    sum
}

#[aoc(day12, part2)]
fn part_two(input: &Vec<(Vec<usize>, Vec<usize>)>) -> u8 {
    1
}

fn new_state_string(i: Vec<char>) -> Vec<usize> {
    i.iter()
        .map(|c| match *c {
            '.' => 0,
            '?' => 1,
            '#' => 2,
            _ => panic!(),
        })
        .collect_vec()
}

#[cached]
fn count(state: Vec<usize>, keys: Vec<usize>) -> usize {
    let total: usize = keys.iter().sum();
    let minimum: usize = state.iter()
        .filter(|&&x| x == 2)
        .count();
    let maximum: usize = state.iter()
        .filter(|&&x| x > 0)
        .count();

    if minimum > total || maximum < total {
        return 0;
    }
    if total == 0 {
        return 0;
    }
    if let Some(f) = state.first() {
        if *f == 0 {
            return count(state[1..].to_vec(), keys);
        } else if *f == 2 {
            let l = keys[0];
            let a = state[..l].iter().all(|&x| x > 0);
            let b = state.len() == l || state[l] < 2;
            if a && b {
                if state.len() == l {
                    return 1;
                }
                return count(state[(l + 1 )..].to_vec(), keys[1..].to_vec());
            }
            return 0;
        }
    }
    let mut v = vec![2];
    v.extend_from_slice(&state[1..]);
    count(state[1..].to_vec(), keys.clone()) + count(v, keys)
}
