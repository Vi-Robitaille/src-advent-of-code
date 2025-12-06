use aoc_runner_derive::{aoc, aoc_generator};

use std::{fmt, vec};

// 
// Note for anyone reading this, this is not meant to look like production code
//   this is an experiment in different ways to handle a crap ton of data
//     with the addition of SplitOffCargoStorage I would use that over my previous implementation
//       purely because it's so much easier to read, understand, and support
//


const ARRAY_SIZE: usize = 2_000_000 * 9; // for meme file sizes
// https://www.reddit.com/r/adventofcode/comments/zd1hqy/comment/iz0avta/?utm_source=share&utm_medium=web2x&context=3

// const ARRAY_SIZE: usize = 60;
const CARGO_SPOTS: usize = 9;
const DEBUG: bool = false;

#[derive(Clone)]
struct CargoInstruction {
    amount: usize,
    source: usize,
    destination: usize,
}

impl fmt::Display for CargoInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Move {} from {} to {}", self.amount, self.source, self.destination)
    }
}

impl CargoInstruction {
    fn new(instruction: &str) -> CargoInstruction {
        let temp: Vec<&str> = instruction.split(' ').collect();
        CargoInstruction {
            amount: temp[1].parse::<usize>().expect("Incorrect amount"),

            // Minus one because these are indexes now
            source: temp[3].parse::<usize>().expect("Incorrect source") -1,
            destination: temp[5].parse::<usize>().expect("Incorrect destination") -1,
        }
    }
}

#[derive(Clone)]
struct CargoStorage {
    cargo: Vec<char>,
    
    // The first available slot in each collumn
    cargo_stack_size: [usize; CARGO_SPOTS],
}

impl fmt::Display for CargoStorage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output: String = String::new();
        for (i, e) in self.cargo_stack_size.iter().enumerate() {
            for x in (i * ARRAY_SIZE)..(i * ARRAY_SIZE + e) {
                output.push(self.cargo[x]);
                output.push('|')
            }
            output.pop();
            output.push('\n');
        }
        write!(f, "{}\nStack sizes: {:?}\n", output, self.cargo_stack_size)
    }
}

impl CargoStorage {
    fn new(cargo_definition: Vec<Vec<char>>) -> CargoStorage {
        let mut cargo: Vec<char> = vec!['!'; ARRAY_SIZE * CARGO_SPOTS];
        let mut cargo_stack_size: [usize; CARGO_SPOTS] = [0; CARGO_SPOTS];

        for (collumn_index, collumn) in cargo_definition.iter().enumerate() {
            for (container_index, container) in collumn.iter().enumerate() {
                let coordinate: usize = ARRAY_SIZE * collumn_index + container_index;
                cargo[coordinate] = *container;
                cargo_stack_size[collumn_index] += 1;
            }
        }         
        CargoStorage {
            cargo, 
            cargo_stack_size,
        }
    }

    // 
    // Reverse: reverse the order of insertion for part 1 or 2
    // Part 1 would reverse (True) while part 2 would not (False)
    // 
    // fn execute_instruction(&mut self, instruction: CargoInstruction, reverse: bool) -> Result<(), &str> {
    fn execute_instruction(&mut self, instruction: &CargoInstruction, reverse: bool) {
        let source_stack_offset: usize = instruction.source * ARRAY_SIZE;

        let source_start_index: usize = source_stack_offset + self.cargo_stack_size[instruction.source] - instruction.amount;
        let source_end_index: usize = source_stack_offset + self.cargo_stack_size[instruction.source] - 1;

        let destination_stack_offset: usize = instruction.destination * ARRAY_SIZE;

        let destination_start_index: usize = destination_stack_offset + self.cargo_stack_size[instruction.destination];
        let destination_end_index: usize = destination_stack_offset + self.cargo_stack_size[instruction.destination] + instruction.amount;

        if DEBUG {
            println!("{}", instruction);
            println!("source_stack_offset: {:?}", source_stack_offset);
            println!("source_start_index: {:?}", source_start_index);
            println!("source_end_index: {:?}", source_end_index);
            println!("destination_stack_offset: {:?}", destination_stack_offset);
            println!("destination_start_index: {:?}", destination_start_index);
            println!("destination_end_index: {:?}", destination_end_index);
            println!();
        }

        self.cargo_stack_size[instruction.destination] += instruction.amount;
        self.cargo_stack_size[instruction.source] -= instruction.amount;
        for (index, element) in (destination_start_index..destination_end_index).enumerate() {
            let source_index: usize;
            if reverse {
                source_index = source_start_index + index;
            } else {
                source_index = source_end_index - index;
            }
            self.cargo[element] = self.cargo[source_index];
            self.cargo[source_index] = '!'
        }

        
    }

    fn get_solution(&self) -> String {
        let mut solution: String = String::new();
        for (i, e) in self.cargo_stack_size.iter().enumerate() {
            let target_index = (i * ARRAY_SIZE) + (e - 1);
            if DEBUG {
                println!("{:?} {:?} {:?}", i, e, i * ARRAY_SIZE);
                println!("Target: {:?}", target_index);
            }
            solution.push(self.cargo[target_index]);
        }
        solution
    }
}


// 
// I learned of split off in vec from a man much smarter than I
// it is much faster, this saves about 5 sec over the other implementation
// 
#[derive(Clone)]
struct SplitOffCargoStorage {
    cargo: Vec<Vec<char>>,
}

impl SplitOffCargoStorage {
    fn new(cargo_definition: Vec<Vec<char>>) -> Self {
        Self { 
            cargo: cargo_definition
        }
    }

    fn execute_instruction(&mut self, instruction: &CargoInstruction, reverse: bool) {
        let source_start_index: usize = self.cargo[instruction.source].len() - instruction.amount;
        let mut crane: Vec<char> = self.cargo[instruction.source].split_off(source_start_index);
        if reverse {
            crane.reverse();
        }
        self.cargo[instruction.destination].append(&mut crane);
    }

    fn get_solution(&self) -> String {
        let mut solution: String = String::new();
        for col in &self.cargo {
            solution.push(*col.last().unwrap_or(&'!'));
        }
        solution
    }
}

impl fmt::Display for SplitOffCargoStorage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nFormat for this is not implemented yet\n")
    }
}

#[aoc_generator(day5)]
fn parse_input_day1(input: &str) -> (SplitOffCargoStorage, Vec<CargoInstruction>) {
    let split: Vec<&str> = input.split("\n\n").collect::<Vec<_>>();

    let mut indexes: [usize; CARGO_SPOTS] = [0; CARGO_SPOTS];
    indexes[0] = 1;

    for i in 1..CARGO_SPOTS {
        indexes[i] = indexes[i - 1] + 4;
    }

    let cargo: SplitOffCargoStorage;
    let mut storage_of_chars: Vec<Vec<char>> = Vec::new();
    for _ in 0..CARGO_SPOTS {
        storage_of_chars.push(vec![]);
    }

    for line in &split[0].split('\n').rev().collect::<Vec<_>>()[1..] {
        let chars = line.chars();
        for (index, element) in indexes.iter().enumerate() {
            if let Some(c) = chars.clone().nth(*element) {
                if c != ' ' {
                    storage_of_chars[index].push(c);
                }
            }
        }
    }

    cargo = SplitOffCargoStorage::new(storage_of_chars);

    let mut instructions: Vec<CargoInstruction> = vec![];
    for line in split[1].split('\n') {
        instructions.push(CargoInstruction::new(line));
    }

    (cargo, instructions)
}

#[aoc(day5, part1)]
fn part_one(input: &(SplitOffCargoStorage, Vec<CargoInstruction>)) -> String {
    let mut cargo = input.0.clone();
    
    if DEBUG {
        println!("{}", cargo);
    }

    let instructions = input.1.clone();
    for instruction in instructions {
        cargo.execute_instruction(&instruction, false);
    }

    if DEBUG {
        println!("{}", cargo);
    }

    cargo.get_solution()
}

#[aoc(day5, part2)]
fn part_two(input: &(SplitOffCargoStorage, Vec<CargoInstruction>)) -> String {
    let mut cargo = input.0.clone();
    
    if DEBUG {
        println!("{}", cargo);
    }

    let instructions = input.1.clone();
    for instruction in instructions {
        cargo.execute_instruction(&instruction, true);
    }

    if DEBUG {
        println!("{}", cargo);
    }

    cargo.get_solution()
}
