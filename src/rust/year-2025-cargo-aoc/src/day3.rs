use aoc_runner_derive::{aoc, aoc_generator};
use colored::Colorize;
use iter_first_max::IterFirstMaxExt;
type Inp = Vec<Vec<u64>>;

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Inp {
    input
        .lines()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap() as u64).collect())
        .collect::<Inp>()
}

#[aoc(day3, part1)]
fn part_one(input: &Inp) -> usize {
    let mut sum = 0;
    for joltage_eval in input {
        let lhs = get_joltage(joltage_eval, 0, joltage_eval.len() - 1);
        let rhs = get_joltage(joltage_eval, lhs.0 + 1, joltage_eval.len());
        // write_joltage(&joltage_eval, &[lhs.0, rhs.0 +1]);
        // println!(" lhs: {} rhs: {} [{}]", lhs.1, rhs.1, lhs.1 * 10 + rhs.1);
        sum += lhs.1 * 10 + rhs.1;
    }
    sum as usize
}

#[allow(unused)]
fn write_joltage(inp: &[u64], colored_indexes: &[usize]) {
    for (i, e) in inp.iter().enumerate() {
        if colored_indexes.contains(&i) {
            print!("{}", e.to_string().red());
        } else {
            print!("{}", e);
        }
    }
}

#[aoc(day3, part2)]
fn part_two(input: &Inp) -> usize {
    let mut sum = 0;
    for joltage_eval in input {
        let j1 = get_joltage(joltage_eval, 0, joltage_eval.len() - 11);
        let j2 = get_joltage(joltage_eval, j1.0 + 1, joltage_eval.len() - 10);
        let j3 = get_joltage(joltage_eval, j2.0 + 1, joltage_eval.len() - 9);
        let j4 = get_joltage(joltage_eval, j3.0 + 1, joltage_eval.len() - 8);
        let j5 = get_joltage(joltage_eval, j4.0 + 1, joltage_eval.len() - 7);
        let j6 = get_joltage(joltage_eval, j5.0 + 1, joltage_eval.len() - 6);
        let j7 = get_joltage(joltage_eval, j6.0 + 1, joltage_eval.len() - 5);
        let j8 = get_joltage(joltage_eval, j7.0 + 1, joltage_eval.len() - 4);
        let j9 = get_joltage(joltage_eval, j8.0 + 1, joltage_eval.len() - 3);
        let j10 = get_joltage(joltage_eval, j9.0 + 1, joltage_eval.len() - 2);
        let j11 = get_joltage(joltage_eval, j10.0 + 1, joltage_eval.len() - 1);
        let j12 = get_joltage(joltage_eval, j11.0 + 1, joltage_eval.len());
        // write_joltage(&joltage_eval, &[j1.0, j2.0, j3.0,j4.0, j5.0, j6.0, j7.0, j8.0, j9.0, j10.0, j11.0, j12.0]);
        // println!();
        sum += j1.1 * 100_000_000_000
            + j2.1 * 10_000_000_000
            + j3.1 * 1_000_000_000
            + j4.1 * 100_000_000
            + j5.1 * 10_000_000
            + j6.1 * 1_000_000
            + j7.1 * 100_000
            + j8.1 * 10_000
            + j9.1 * 1_000
            + j10.1 * 100
            + j11.1 * 10
            + *j12.1;
    }
    sum as usize
}

fn get_joltage(inp: &[u64], start: usize, end: usize) -> (usize, &u64) {
    let r = inp[start..end]
        .iter()
        .enumerate()
        .first_max_by(|x, y| x.1.cmp(y.1))
        .unwrap();
    (start + r.0, r.1)
}
