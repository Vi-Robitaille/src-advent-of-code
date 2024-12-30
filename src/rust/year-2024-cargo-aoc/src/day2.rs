use aoc_runner_derive::{aoc, aoc_generator};

type I = isize;
type Inp = Vec<Vec<I>>;

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Inp {
    input
        .lines()
        .map(|x| x.split(' ').map(|y| y.parse::<I>().unwrap()).collect())
        .collect()
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    Ascending,
    Descending,
    Unset,
}

#[aoc(day2, part1)]
fn part_one(input: &Inp) -> usize {
    input
        .iter()
        .filter_map(|e| eval_entry(e).then_some(1))
        .sum()
}

fn eval_entry(e: &[I]) -> bool {
    let mut state = State::Unset;
    for j in e.windows(2) {
        match (j[0], j[1], state) {
            (a, b, _) if a.abs_diff(b) > 3 => return false,
            (a, b, _) if a == b => return false,
            // state is Ascending, pair is Descending -> Unsafe
            (a, b, State::Ascending) if (a > b) => return false,
            // state is Descending, pair is Descending -> Safe
            (a, b, State::Descending) if (a > b) => {}
            // state is Unset, pair is Descending -> Set to Descending
            (a, b, State::Unset) if (a > b) => state = State::Descending,
            // state is Ascending, pair is Ascending -> Safe
            (a, b, State::Ascending) if (a < b) => {}
            // state is Descending, pair is Ascending -> Safe
            (a, b, State::Descending) if (a < b) => return false,
            // state is Unset, pair is Ascending -> Set to Ascending
            (a, b, State::Unset) if (a < b) => state = State::Ascending,
            _ => panic!("how??"),
        }
    }
    true
}

#[aoc(day2, part2)]
fn part_two(input: &Inp) -> usize {
    input
        .iter()
        .filter_map(|e| eval_entry_part_twooo(e).then_some(1))
        .sum()
}

fn eval_entry_part_twooo(e: &[I]) -> bool {
    !(0..e.len())
        .filter(|i| {
            let mut a = e.to_vec();
            let _ = a.remove(*i);
            eval_entry(&a)
        })
        .collect::<Vec<_>>()
        .is_empty()
}
