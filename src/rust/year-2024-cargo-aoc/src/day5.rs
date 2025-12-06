use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

type Inp = (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>);

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Inp {
    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut f = input.split("\n\n");
    let rules_half = f.next().unwrap();
    let books_half = f.next().unwrap();

    for line in rules_half.lines() {
        let rule = line
            .split('|')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();
        rules.entry(rule[0]).or_default().push(rule[1]);
    }

    let books = books_half
        .lines()
        .map(|e| e.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();
    (rules, books)
}

fn is_valid_book(book: &[usize], rules: &HashMap<usize, Vec<usize>>) -> bool {
    for (idx, page1) in book.iter().rev().enumerate() {
        for page2 in book.iter().rev().skip(idx) {
            if let Some(v) = rules.get(page1) {
                if v.contains(page2) {
                    return false;
                }
            }
        }
    }
    true
}

#[aoc(day5, part1)]
fn part_one(input: &Inp) -> usize {
    let (rules, books) = input;
    books
        .iter()
        .filter_map(|book| is_valid_book(book, rules).then_some(book[book.len().div_ceil(2) - 1]))
        .sum()
}

#[aoc(day5, part2)]
fn part_two(input: &Inp) -> usize {
    let (rules, books) = input;
    books
        .iter()
        .filter_map(|book| {
            (!is_valid_book(book, rules))
                .then_some(reorder_pages(book, rules)[book.len().div_ceil(2) - 1])
        })
        .sum()
}

fn reorder_pages(book: &[usize], rules: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut r = book.to_vec();
    r.sort_by(|&a, &b| {
        if rules.get(&a).is_some_and(|d| d.contains(&b)) {
            return std::cmp::Ordering::Less;
        }
        if rules.get(&b).is_some_and(|d| d.contains(&a)) {
            return std::cmp::Ordering::Greater;
        }
        std::cmp::Ordering::Equal
    });
    r
}
