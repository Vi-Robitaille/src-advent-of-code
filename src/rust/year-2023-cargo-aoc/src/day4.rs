use aoc_runner_derive::{aoc, aoc_generator};

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref NUMBER_REGEX: Regex = Regex::new(r"\d+").unwrap();
}

#[aoc_generator(day4)]
fn parse_input_day1(input: &str) -> Vec<(Vec<usize>, Vec<usize>)> {
    input
        .lines()
        .map(|x| x.split_once(':').unwrap().1)
        .map(|x| x.split_once('|').unwrap())
        .map(|(lhs, rhs)| {
            let winning_numbers = NUMBER_REGEX
                .find_iter(lhs)
                .map(|x| x.as_str().parse::<usize>().unwrap())
                .collect();
            let card_numbers = NUMBER_REGEX
                .find_iter(rhs)
                .map(|x| x.as_str().parse::<usize>().unwrap())
                .collect();
            (winning_numbers, card_numbers)
        })
        .collect()
}

#[aoc(day4, part1)]
fn part_one(input: &Vec<(Vec<usize>, Vec<usize>)>) -> usize {
    let mut result = Vec::new();
    for (winning_numbers, card_numbers) in input {
        result.push(solve_game(solve_game_matching_numbers(
            winning_numbers,
            card_numbers,
        )));
    }
    result.iter().sum()
}

#[aoc(day4, part2)]
fn part_two(input: &Vec<(Vec<usize>, Vec<usize>)>) -> usize {
    // How many wins per game
    let mut game_scores: HashMap<usize, usize> = HashMap::new();
    for (index, (winning_numbers, card_numbers)) in input.iter().enumerate() {
        game_scores.insert(
            index,
            solve_game_matching_numbers(winning_numbers, card_numbers),
        );
    }

    // Calculate the number of proceeding wins
    //  num wins -> (i + 1 .. =num wins + 1) += 1
    let mut total_cards: HashMap<usize, usize> = HashMap::new();

    for i in 0..input.len() {
        total_cards.insert(i, 1);
    }

    // for each score in the map of cards -> scores
    //  iterate over the map of total cards

    for i in 0..input.len() {
        let current_game_wins = if let Some(index) = total_cards.get(&i) {
            *index
        } else {
            continue;
        };

        let current_game_score = if let Some(index) = game_scores.get(&i) {
            *index
        } else {
            continue;
        };

        for _ in 0..current_game_wins {
            for j in 0..current_game_score {
                let game_index = i + j + 1;
                if let Some(game_count) = total_cards.get_mut(&game_index) {
                    *game_count += 1;
                }
            }
        }
    }

    // Take the count of scratch cards and sum it
    total_cards.values().sum()
}

fn solve_game_matching_numbers(winning_numbers: &[usize], card_numbers: &[usize]) -> usize {
    let mut set: HashSet<usize> = HashSet::new();
    for i in winning_numbers.iter() {
        set.insert(*i);
    }
    let mut score: usize = 0;
    for num in card_numbers {
        if set.get(num).is_some() {
            score += 1;
        }
    }
    score
}

///
/// considering there arent sqrt(100M) input numbers per line i think this is a safe guard.
///
fn solve_game(score: usize) -> usize {
    match score {
        0 => 0,
        1 => 1,
        i if i < 100_000_000 => usize::pow(2, (score - 1) as u32),
        _ => panic!("How?"),
    }
}
