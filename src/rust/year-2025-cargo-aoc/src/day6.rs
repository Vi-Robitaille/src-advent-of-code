use std::iter::StepBy;

use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;
type Inp = (Vec<usize>, Vec<String>);
type InpPartTwo = Vec<(Vec<usize>, String)>;

#[aoc_generator(day6, part1)]
fn parse_input_part_one(input: &str) -> Inp {
    let mut digits: Vec<usize> = vec![];
    let mut ops: Vec<String> = vec![];
    for (key, chunk) in &input.chars().chunk_by(|x| *x != '\n' && *x != ' ') {
        if !key {
            continue;
        }
        let c = chunk.collect::<String>();
        if let Ok(d) = c.parse::<usize>() {
            digits.push(d);
        } else {
            ops.push(c);
        }
    }

    (digits, ops)
}

#[aoc_generator(day6, part2)]
fn parse_input_part_two(input: &str) -> InpPartTwo {
    let mut lines_iter = input.lines();

    let ops: Vec<String> = lines_iter
        .next_back()
        .unwrap()
        .chars()
        .rev()
        .filter_map(|x| match x != '\n' && x != ' ' {
            true => Some(x.to_string()),
            false => None,
        })
        .collect();

    let x = transpose(
        lines_iter
            .map(|x| x.chars().rev().collect::<Vec<char>>())
            .collect_vec(),
    )
    .into_iter()
    .chunk_by(|x| x.iter().all(|y| *y == ' '))
    .into_iter()
    .filter_map(|(key, chunk)| {
        (!key).then(|| {
            chunk
                .map(|x| {
                    x.iter()
                        .filter(|y| **y != ' ')
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
    })
    .zip(ops)
    .collect::<Vec<(Vec<usize>, String)>>();
    x
}

#[aoc(day6, part1)]
fn part_one(input: &Inp) -> usize {
    let l = input.1.len();
    let mut s = 0;
    for (idx, op) in input.1.iter().enumerate() {
        let nums = get_numbers(&input.0, idx, l);

        // .product() and .sum() dont work on &_ :(
        s += match op.as_str() {
            "+" => nums.sum::<usize>(),
            "*" => nums.product::<usize>(),
            _ => panic!(),
        };
    }
    s
}

#[aoc(day6, part2)]
fn part_two(input: &InpPartTwo) -> usize {
    input
        .iter()
        .map(|(math, op)| match op.as_str() {
            "+" => math.iter().sum::<usize>(),
            "*" => math.iter().product::<usize>(),
            _ => panic!(),
        })
        .sum()
}

fn get_numbers(input: &[usize], idx: usize, offset: usize) -> StepBy<std::slice::Iter<'_, usize>> {
    input[idx..].iter().step_by(offset)
}

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
