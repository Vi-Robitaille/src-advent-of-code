use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(dayX)]
fn parse_input_day1(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {} )
        .collect::<Vec<_>>()
}

#[aoc(dayX, part1)]
fn part_one(input: &Vec<i32>) -> i32 {
    1
}

#[aoc(dayX, part2)]
fn part_two(input: &Vec<i32>) -> i32 {
    1
}

#[cfg(test)]
#[test]
fn test_find_2020_entries() {
    let test_vec = vec![1721, 979, 366, 299, 675, 1456];

    assert_eq!(1, part_one(&test_vec))
}