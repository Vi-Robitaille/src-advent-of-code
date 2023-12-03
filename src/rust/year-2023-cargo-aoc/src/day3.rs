use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{multizip, Zip, Itertools, chain};

const MAX_SIZE: usize = 140;

// Non debug version
// const MAX_SIZE: usize = 140;


#[aoc_generator(day3)]
fn parse_input_day1(input: &str) -> String {
    // pls dont look
    
    let padding: String = ['.'; MAX_SIZE].iter().collect();
    let mutated = [
            padding.clone(), 
            String::from("\n"), 
            input.to_string(), 
            String::from("\n"), 
            padding
        ]
        .concat();
    mutated

}

#[aoc(day3, part1)]
fn part_one(input: &String) -> usize {
    input
        .lines()
        .collect::<Vec<&str>>()
        .windows(3)
        .map(|x| {
            let groupings = x[1].chars()
                .enumerate()
                .group_by(|(_i, e)| e.is_digit(10))
                .into_iter()
                .filter_map(|(key, group)| key.then(|| group.collect()))
                .collect::<Vec<Vec<(usize, char)>>>();

            // [(0, '4'), (1, '6'), (2, '7')],
            // [(5, '1'), (6, '1'), (7, '4')]

            groupings
                .iter()
                .filter_map(|m| {
                    let mut first = m.first().expect("Could not grab the first in `for group in groupings`").0;
                    let mut last = m.last().expect("Could not grab the last in `for group in groupings`").0 + 1;
    
                    
                    let numeral = match x[1][first..last].parse::<usize>() {
                        Err(_e) => {
                            println!("Could not parse {:?}", String::from(&x[1][first..last+1]));
                            panic!()
                        },
                        Ok(x) => x,
                    };

                    if (1..MAX_SIZE -1).contains(&first) { first = first - 1; }
                    if (1..MAX_SIZE -1).contains(&last) { last = last + 1; }                    

                    let a = x[0][first..last].chars().clone();
                    let b = x[1][first..last].chars().clone();
                    let c = x[2][first..last].chars().clone();

                    let is_near_symbol = chain(a, chain(b, c))
                        .any(|f| {
                            match f {
                                i if i.is_digit(10) => false,
                                i if i == '.' => false,
                                _ => true,
                            }
                        });
                    is_near_symbol.then(|| numeral)
                })
                .collect::<Vec<usize>>()
        })
        .flatten()
        .sum()
}

#[aoc(day3, part2)]
fn part_two(input: &String) -> u8 {
    1
}

