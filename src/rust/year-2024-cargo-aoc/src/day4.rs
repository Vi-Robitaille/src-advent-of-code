use aoc_runner_derive::{aoc, aoc_generator};

type Inp = Vec<Vec<char>>;

const SEARCH: [char; 4] = ['X', 'M', 'A', 'S'];
const SEARCH_REVERSE: [char; 4] = ['S', 'A', 'M', 'X'];

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Inp {
    input.lines().map(|x| x.chars().collect()).collect()
}

#[aoc(day4, part1)]
fn part_one(input: &Inp) -> usize {
    let mut count = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == 'X' {
                count += test_section(input, y, x)
            }
        }
    }
    count
}

fn test_section(ss: &Inp, y: usize, x: usize) -> usize {
    fn inc(a: [char; 4], count: &mut usize) {
        if a == SEARCH || a == SEARCH_REVERSE {
            *count += 1;
        }
    }
    let mut count = 0;
    // Top
    if y.checked_sub(SEARCH.len() - 1).is_some() {
        let a = [ss[y][x], ss[y - 1][x], ss[y - 2][x], ss[y - 3][x]];
        inc(a, &mut count);
    }
    // Top Right
    if y.checked_sub(SEARCH.len() - 1).is_some() && x + SEARCH.len() <= ss[0].len() {
        let a = [
            ss[y][x],
            ss[y - 1][x + 1],
            ss[y - 2][x + 2],
            ss[y - 3][x + 3],
        ];
        inc(a, &mut count);
    }
    // Right
    if x + SEARCH.len() <= ss[0].len() {
        let a = [ss[y][x], ss[y][x + 1], ss[y][x + 2], ss[y][x + 3]];
        inc(a, &mut count);
    }
    // Bottom Right
    if y + SEARCH.len() <= ss.len() && x + SEARCH.len() <= ss[0].len() {
        let a = [
            ss[y][x],
            ss[y + 1][x + 1],
            ss[y + 2][x + 2],
            ss[y + 3][x + 3],
        ];
        inc(a, &mut count);
    }
    // Bottom
    if y + SEARCH.len() <= ss.len() {
        let a = [ss[y][x], ss[y + 1][x], ss[y + 2][x], ss[y + 3][x]];
        inc(a, &mut count);
    }
    // Top Left
    if y.checked_sub(SEARCH.len() - 1).is_some() && x.checked_sub(SEARCH.len() - 1).is_some() {
        let a = [
            ss[y][x],
            ss[y - 1][x - 1],
            ss[y - 2][x - 2],
            ss[y - 3][x - 3],
        ];
        inc(a, &mut count);
    }
    // Left
    if x.checked_sub(SEARCH.len() - 1).is_some() {
        let a = [ss[y][x], ss[y][x - 1], ss[y][x - 2], ss[y][x - 3]];
        inc(a, &mut count);
    }
    // Bottom Left
    if y + SEARCH.len() <= ss.len() && x.checked_sub(SEARCH.len() - 1).is_some() {
        let a = [
            ss[y][x],
            ss[y + 1][x - 1],
            ss[y + 2][x - 2],
            ss[y + 3][x - 3],
        ];
        inc(a, &mut count);
    }
    count
}

#[aoc(day4, part2)]
fn part_two(input: &Inp) -> usize {
    let mut count = 0;
    for y in 1..input.len() - 1 {
        for x in 1..input[y].len() - 1 {
            if input[y][x] == 'A' {
                count += test_section_part_two(input, y, x)
            }
        }
    }
    count
}

fn test_section_part_two(ss: &Inp, y: usize, x: usize) -> usize {
    let mut count = 0;
    let a = [
        ss[y - 1][x - 1],
        ss[y - 1][x + 1],
        ss[y][x],
        ss[y + 1][x - 1],
        ss[y + 1][x + 1],
    ];
    let b = ['M', 'S', 'A', 'M', 'S'];
    let c = ['S', 'S', 'A', 'M', 'M'];
    let d = ['S', 'M', 'A', 'S', 'M'];
    let e = ['M', 'M', 'A', 'S', 'S'];
    if a == b || a == c || a == d || a == e {
        count += 1;
    }
    count
}
// 2571
// 3272 Low
// 3350 high
