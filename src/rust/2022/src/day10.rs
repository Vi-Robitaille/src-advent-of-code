use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn parse_input_day10(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| Instruction::new(line) )
        .collect::<Vec<_>>()
}

struct Instruction {
    opcode: String,
    value: isize
}

impl Instruction {
    fn new(inp: &str) -> Self {
        let mut split = inp.split(' ');
        let opcode: String = split.next().unwrap().to_string();        
        let value: isize = split.next().unwrap_or("0").parse::<isize>().unwrap_or(0);
        if opcode == "noop" && isize::abs(value) > 0 {
            panic!("Non zero noop.")
        }
        if opcode == "addx" && value == 0 {
            panic!("Addx with zero as X.")
        }
        Self {
            opcode,
            value,
        }
    }
    fn is_non_noop(&self) -> bool {
        if self.opcode == "noop".to_string() {
            return false;
        }
        true
    }
}

fn do_cycle_part1(register: isize, evaluation_cycles: &Vec<isize>, cycles: &mut isize, result: &mut isize) {
    if evaluation_cycles.contains(&cycles) {
        *result += *cycles * register;
    }
    *cycles += 1
}

#[aoc(day10, part1)]
fn part_one(input: &Vec<Instruction>) -> isize {
    let (mut cycles, mut register, mut result): (isize, isize, isize) = (1, 1, 0);
    let evaluation_cycles: Vec<isize> = vec![20, 60, 100, 140, 180, 220];
    for instruction in input {
        do_cycle_part1(register, &evaluation_cycles, &mut cycles, &mut result);
        if instruction.is_non_noop() {
            do_cycle_part1(register, &evaluation_cycles, &mut cycles, &mut result);
            register += instruction.value;
        }
    }
    result
}

fn do_cycle_part2(register:isize, width: isize, cycles: &mut isize, result: &mut String) {
    let index = *cycles % width;
    if (register..register+3).contains(&index) {
        result.push('█');
    } else {
        result.push(' ');
    }
    *cycles += 1;
}

#[aoc(day10, part2)]
fn part_two(input: &Vec<Instruction>) -> String {
    let (mut cycles, mut register, mut result): (isize, isize, String) = (1, 1, '\n'.to_string());
    let width: isize = 40;
    for instruction in input {
        do_cycle_part2(register, width, &mut cycles, &mut result);
        if instruction.is_non_noop() {
            do_cycle_part2(register, width, &mut cycles, &mut result);
            register += instruction.value;
        }
        if cycles % width == 1 {
            result.push('\n');
        }
    }
    result
}

#[cfg(test)]
#[test]
fn test_one() {
    let test_vec = "addx 15
    addx -11
    addx 6
    addx -3
    addx 5
    addx -1
    addx -8
    addx 13
    addx 4
    noop
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx -35
    addx 1
    addx 24
    addx -19
    addx 1
    addx 16
    addx -11
    noop
    noop
    addx 21
    addx -15
    noop
    noop
    addx -3
    addx 9
    addx 1
    addx -3
    addx 8
    addx 1
    addx 5
    noop
    noop
    noop
    noop
    noop
    addx -36
    noop
    addx 1
    addx 7
    noop
    noop
    noop
    addx 2
    addx 6
    noop
    noop
    noop
    noop
    noop
    addx 1
    noop
    noop
    addx 7
    addx 1
    noop
    addx -13
    addx 13
    addx 7
    noop
    addx 1
    addx -33
    noop
    noop
    noop
    addx 2
    noop
    noop
    noop
    addx 8
    noop
    addx -1
    addx 2
    addx 1
    noop
    addx 17
    addx -9
    addx 1
    addx 1
    addx -3
    addx 11
    noop
    noop
    addx 1
    noop
    addx 1
    noop
    noop
    addx -13
    addx -19
    addx 1
    addx 3
    addx 26
    addx -30
    addx 12
    addx -1
    addx 3
    addx 1
    noop
    noop
    noop
    addx -9
    addx 18
    addx 1
    addx 2
    noop
    noop
    addx 9
    noop
    noop
    noop
    addx -1
    addx 2
    addx -37
    addx 1
    addx 3
    noop
    addx 15
    addx -21
    addx 22
    addx -6
    addx 1
    noop
    addx 2
    addx 1
    noop
    addx -10
    noop
    noop
    addx 20
    addx 1
    addx 2
    addx 2
    addx -6
    addx -11
    noop
    noop
    noop";

    assert_eq!(13140, part_one(&test_vec))
}