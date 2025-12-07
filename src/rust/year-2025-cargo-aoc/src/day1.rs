use std::ops::AddAssign;

use aoc_runner_derive::{aoc, aoc_generator};

type Inp = Vec<isize>;

struct State {
    head_position: isize,
    zero_count: usize,
}

impl State {
    fn new(head_position: isize, zero_count: usize) -> Self {
        Self {
            head_position,
            zero_count,
        }
    }

    fn count_clicks_but_fuck_you(&mut self, rhs: isize) {
        let sig = rhs.signum();
        if sig == 0 {
            return;
        }
        *self += sig;
        if self.head_position == 0 {
            self.zero_count += 1
        };
        self.count_clicks_but_fuck_you(rhs - sig);
    }

    #[allow(unused)]
    fn count_clicks(&mut self, rhs: isize) {
        let wraps =
            ((self.head_position + rhs).div_euclid(100) - self.head_position.div_euclid(100)).abs();
        println!("{wraps}");
        *self += rhs;
        self.zero_count += wraps.unsigned_abs();
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new(50, 0)
    }
}

impl AddAssign<isize> for State {
    fn add_assign(&mut self, rhs: isize) {
        self.head_position = (self.head_position + rhs).rem_euclid(100);
    }
}

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Inp {
    input
        .lines()
        .map(|l| {
            let (dir, val) = l.split_at(1);
            let value = val.parse::<isize>().expect("Invalid number");
            let direction = match dir {
                "L" => -1,
                "R" => 1,
                _ => panic!("Invalid direction"),
            };

            value * direction
        })
        .collect()
}

#[aoc(day1, part1)]
fn part_one(input: &Inp) -> usize {
    let mut state: State = State::default();
    input.iter().for_each(|i| {
        state += *i;
        state.zero_count += if state.head_position == 0 { 1 } else { 0 };
    });

    state.zero_count
}

#[aoc(day1, part2)]
fn part_two(input: &Inp) -> usize {
    let mut state: State = State::default();
    input.iter().for_each(|i| {
        state.count_clicks_but_fuck_you(*i);
    });

    state.zero_count
}
