use std::cmp::Ordering;

use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;

#[aoc_generator(day7)]
fn parse_input_day1(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|x| {
            let mut pair = x.split(' ').take(2);
            let lhs: [char; 5] = pair
                .next()
                .unwrap()
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .unwrap();
            let rhs: usize = pair.next().unwrap().parse::<usize>().unwrap();

            Hand::new(lhs, rhs)
        })
        .collect_vec()
}

#[aoc(day7, part1)]
fn part_one(input: &[Hand]) -> usize {
    let mut input = input.to_owned();
    input.sort();
    input.iter().enumerate().map(|(i, e)| (i + 1) * e.bid).sum()
}

#[aoc(day7, part2)]
fn part_two(_input: &[Hand]) -> usize {
    1
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
    power: PowerStates,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.power, &self.cards).cmp(&(other.power, &other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn new(c: [char; 5], bid: usize) -> Self {
        let mut c2 = c;
        c2.sort();

        let mut character_counts: Vec<usize> = c2
            .iter()
            .group_by(|&x| x)
            .into_iter()
            .map(|(_k, v)| v.count())
            .collect();

        character_counts.sort();

        let cards: [Card; 5] = c
            .iter()
            .map(|x| Card::from(*x))
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap();

        // println!("{:?} -> {:?}", c2, character_counts);

        let power = if character_counts.len() == 1 {
            PowerStates::FiveOfAKind
        } else {
            match character_counts
                .iter()
                .rev()
                .take(2)
                .collect_tuple::<(&usize, &usize)>()
            {
                Some((&4, &1)) => PowerStates::FourOfAKind,
                Some((&3, &2)) => PowerStates::FullHouse,
                Some((&3, &1)) => PowerStates::ThreeOfAKind,
                Some((&2, &2)) => PowerStates::TwoPair,
                Some((&2, &1)) => PowerStates::OnePair,
                Some((&1, &1)) => PowerStates::HighCard,
                Some(_) => panic!(),
                None => panic!(),
            }
        };
        Hand { cards, bid, power }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
enum PowerStates {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            '1' => Card::One,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    One = 1,
}
