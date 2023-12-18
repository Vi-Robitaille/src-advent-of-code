use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{chain, Itertools};
use std::collections::HashMap;

const MAX_SIZE: usize = 140;

// Non debug version
// const MAX_SIZE: usize = 140;

///
/// I had a lot of fun seeing how deep i could chain iters
///   Of course this is cursed, but live with it!
///    A good friend showed me `group_by` from itertools and
///     I feel it really took this solution to another level
///

#[aoc_generator(day3)]
fn parse_input_day1(input: &str) -> String {
    // pls dont look

    let padding: String = ['.'; MAX_SIZE].iter().collect();

    [
        padding.clone(),
        String::from("\n"),
        input.to_string(),
        String::from("\n"),
        padding,
    ]
    .concat()
}

#[aoc(day3, part1)]
fn part_one(input: &str) -> usize {
    input
        .lines()
        .collect::<Vec<&str>>()
        .windows(3)
        .flat_map(|x| {
            let groupings = x[1]
                .chars()
                .enumerate()
                .group_by(|(_i, e)| e.is_ascii_digit())
                .into_iter()
                .filter_map(|(key, group)| key.then(|| group.collect()))
                .collect::<Vec<Vec<(usize, char)>>>();

            // [(0, '4'), (1, '6'), (2, '7')],
            // [(5, '1'), (6, '1'), (7, '4')]

            groupings
                .iter()
                .filter_map(|m| {
                    let mut first = m
                        .first()
                        .expect("Could not grab the first in `for group in groupings`")
                        .0;
                    let mut last = m
                        .last()
                        .expect("Could not grab the last in `for group in groupings`")
                        .0
                        + 1;

                    let numeral = match x[1][first..last].parse::<usize>() {
                        Err(_e) => {
                            println!("Could not parse {:?}", String::from(&x[1][first..last + 1]));
                            panic!()
                        }
                        Ok(x) => x,
                    };

                    if (1..MAX_SIZE - 1).contains(&first) {
                        first -= 1;
                    }
                    if (1..MAX_SIZE - 1).contains(&last) {
                        last += 1;
                    }

                    let a = x[0][first..last].chars().clone();
                    let b = x[1][first..last].chars().clone();
                    let c = x[2][first..last].chars().clone();

                    let is_near_symbol = chain(a, chain(b, c)).any(|f| match f {
                        i if i.is_ascii_digit() => false,
                        '.' => false,
                        _ => true,
                    });
                    is_near_symbol.then_some(numeral)
                })
                .collect::<Vec<usize>>()
        })
        .sum()
}

#[aoc(day3, part2)]
fn part_two(input: &str) -> usize {
    let r = input
        .lines()
        .enumerate()
        .collect::<Vec<(usize, &str)>>()
        .windows(3)
        .flat_map(|x| {
            // (Line_id, line)
            let groupings = x[1]
                .1
                .chars()
                .enumerate()
                .group_by(|(_i, e)| e.is_ascii_digit())
                .into_iter()
                .filter_map(|(key, group)| key.then(|| group.collect()))
                .collect::<Vec<Vec<(usize, char)>>>();

            // line_id = x[X].0
            // groupings is a vec of the X position in the string and teh numeric character at that position
            // [(0, '4'), (1, '6'), (2, '7')],
            // [(5, '1'), (6, '1'), (7, '4')]

            groupings
                .iter()
                .filter_map(|m| {
                    // The top left coord of the current zone of influence
                    let mut zero_x = m.first().unwrap().0;
                    if zero_x == 0 {
                        zero_x += 1;
                    }
                    let zero_y = x[0].0;

                    let mut first = m
                        .first()
                        .expect("Could not grab the first in `for group in groupings`")
                        .0;
                    let mut last = m
                        .last()
                        .expect("Could not grab the last in `for group in groupings`")
                        .0
                        + 1;

                    let numeral = match x[1].1[first..last].parse::<usize>() {
                        Err(_e) => {
                            println!(
                                "Could not parse {:?}",
                                String::from(&x[1].1[first..last + 1])
                            );
                            panic!()
                        }
                        Ok(x) => x,
                    };

                    if (1..MAX_SIZE - 1).contains(&first) {
                        first -= 1;
                    }
                    if (1..MAX_SIZE - 1).contains(&last) {
                        last += 1;
                    }

                    let positions = chain(
                        x[0].1[first..last].chars().clone(),
                        chain(
                            x[1].1[first..last].chars().clone(),
                            x[2].1[first..last].chars().clone(),
                        ),
                    )
                    .enumerate()
                    .filter_map(|(i, ch)| {
                        (ch == '*')
                            .then_some((zero_x + (i % last - first), zero_y + (i / last - first)))
                    })
                    .collect::<Vec<Point>>();
                    (!positions.is_empty()).then_some((positions, numeral))
                })
                .collect::<Vec<(Vec<Point>, usize)>>()
        })
        .collect::<Vec<(Vec<Point>, usize)>>();

    let mut map: HashMap<(usize, usize), usize> = HashMap::new();
    let mut result: Vec<usize> = vec![];
    for (p, i) in r.iter() {
        for (x, y) in p {
            // If this (x, y) coord is in the map then return it and append
            //  the product of it and the current iteration elem to the result
            if let Some(inner_value) = map.get(&(*x, *y)) {
                result.push(inner_value * i);
            } else {
                map.insert((*x, *y), *i);
            }
        }
    }
    result.iter().sum()
}

type Point = (usize, usize);
