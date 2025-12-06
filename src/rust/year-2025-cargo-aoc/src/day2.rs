use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Inp = Vec<(usize, usize)>;

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Inp {
    input
        .split(',')
        .map(|x| {
            let mut cont = x.split('-');
            let a = cont.next().unwrap().parse::<usize>().unwrap();
            let b = cont.next().unwrap().parse::<usize>().unwrap();
            (a, b)
        })
        .collect()
}

#[aoc(day2, part1)]
fn part_one(input: &Inp) -> usize {
    let mut sum = 0;
    for (a, b) in input {
        for element in *a..=*b {
            let elem_str = element.to_string();
            let len_elem_str = elem_str.len();

            let first_half = &elem_str[..len_elem_str / 2];
            let second_half = &elem_str[len_elem_str / 2..];

            if first_half == second_half {
                sum += element;
            }
        }
    }
    sum
}

#[aoc(day2, part2)]
fn part_two(input: &Inp) -> usize {
    let mut sum = 0;
    for (a, b) in input {
        for element in *a..=*b {
            let s = element.to_string();
            if s.len() < 2 {
                continue;
            }

            let s_vec = s.chars().collect::<Vec<char>>();

            if s_vec.windows(2).all(|x| x[0] == x[1]) {
                sum += element;
                continue;
            }

            for div in divisors::get_divisors(s.len())
                .iter()
                .filter(|x| **x != s.len())
            {
                if *div == s.len() {
                    continue;
                }
                let r = s_vec
                    .iter()
                    .chunks(*div)
                    .into_iter()
                    .map(|x| x.collect::<String>().parse::<usize>().unwrap())
                    .tuple_windows::<(usize, usize)>()
                    .all(|x| x.0 == x.1);
                if r {
                    sum += element;
                    break;
                }
            }
        }
    }
    sum
}
