use btree_range_map::RangeSet;

use aoc_runner_derive::{aoc, aoc_generator};

type Inp = (RangeSet<u64>, Vec<u64>);

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Inp {
    let mut i_split = input.split("\n\n");
    let ranges = i_split
        .next()
        .unwrap()
        .lines()
        .map(|x| {
            let mut spl = x.split("-");
            let a = spl.next().unwrap().parse::<u64>().unwrap();
            let b = spl.next().unwrap().parse::<u64>().unwrap();
            a..=b
        })
        .collect::<RangeSet<u64>>();
    let ingredients = i_split
        .next()
        .unwrap()
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    (ranges, ingredients)
}

#[aoc(day5, part1)]
fn part_one(input: &Inp) -> usize {
    input.1.iter().filter(|x| input.0.contains(**x)).count()
}

#[aoc(day5, part2)]
fn part_two(input: &Inp) -> usize {
    input.0.len().try_into().unwrap()
}

// 45.73.49.28
// 207.164.151.60
