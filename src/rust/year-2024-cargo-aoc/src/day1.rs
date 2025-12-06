use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Vec<(usize, usize)> {
    input
        .split('\n')
        .map(|x| {
            (
                x[0..5].parse().expect("num 1 failed to parse"),
                x[8..].parse().expect("num 2 failed to parse"),
            )
        })
        .collect::<Vec<(usize, usize)>>()
}

#[aoc(day1, part1)]
fn part_one(input: &[(usize, usize)]) -> usize {
    let mut lhs = vec![];
    let mut rhs = vec![];
    for i in input {
        lhs.push(i.0);
        rhs.push(i.1);
    }
    lhs.sort();
    rhs.sort();
    println!("{:?}", lhs[0]);
    let mut sum = 0;
    for i in 0..lhs.len() {
        sum += lhs[i].abs_diff(rhs[i]);
    }
    sum
}

#[aoc(day1, part2)]
fn part_two(input: &[(usize, usize)]) -> usize {
    let mut lhs = vec![];
    let mut rhs = vec![];
    for i in input {
        lhs.push(i.0);
        rhs.push(i.1);
    }
    lhs.sort();
    rhs.sort();

    let hm = rhs.into_iter().counts();
    lhs.iter().map(|x| hm.get(x).unwrap_or(&0) * x).sum()
}
