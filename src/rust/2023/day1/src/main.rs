
use std::fs;


fn main() {
    let data = load_data();
    let part1_solution = part1(&data);
    let part2_solution = part2(&data);
    println!("Part 1: {}", part1_solution);
    println!("Part 2: {}", part2_solution);
}


fn part1(inp: &Vec<String>) -> usize {
    inp.iter()
        .map(|x| {
            let left_right = from_one_end(x);
            let right_left = from_one_end(&x.chars().rev().collect::<String>());
            left_right * 10 + right_left
        })
        .sum()
}

fn part2(inp: &Vec<String>) -> usize {
    inp.iter()
        .map(|lamb| {
            let x = &BIG_REPLACE(lamb);
            let y = &x.chars().rev().collect::<String>();
            let left_right = from_one_end(x);
            let right_left = from_one_end(y);
            left_right * 10 + right_left
        })
        .sum()
}

fn from_one_end(inp: &String) -> usize {
     for c in inp.chars().into_iter() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap() as usize;
        }
     }
    0
}

/// Gotta make sure its a yukky solution!
#[allow(non_snake_case)]
fn BIG_REPLACE(inp: &String) -> String {
    inp
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
        .replace("zero", "zero0zero")
}

fn load_data() -> Vec<String> {
    fs::read_to_string("day1.txt")
        .expect("A FUCKY WUCKY!")
        .split('\n')
        .map(|x| String::from(x))
        .collect::<Vec<String>>()
}


