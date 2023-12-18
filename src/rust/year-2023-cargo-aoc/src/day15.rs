use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::fmt;
use std::collections::HashMap;


///
/// Things we need to keep track of
/// order of lenses
/// 
/// Things we need to be able to do
/// Fast label lookup
/// insert / remove and maintain ordering without modifying all other elems in the structure
/// update a lens via label
/// 

/// Insertion
/// Test update
/// if not updating, push to back
/// 
/// Remove
///  Change to ""
/// 
/// Boxes [Vec<&str>; 256]
/// this will store the label of the lens as a placeholder
///  lens labels are unique per box
/// 
/// BTreeMap?
/// HashMap?
/// Type<(usize, &str), usize>
///     <(BoxId, Label), Power>
/// 
/// If we know the box id and label (which we get from out input)
///  we can check if the hash structure contains that combination
/// 
/// If it does, we go to that box and perform our action on the box.
/// 
/// Calculating solution
/// 
/// for box in boxes {
///   box.iter()
///     .enumerate()
///     .filter_map(|(i, s)| s.is_empty().then_some(hashmap.get((i, s)) * i))
///     .sum()
/// }
/// 
/// This will yield an ordered list per box of lens powers
/// 

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .split(',')
        .map(|c| c.chars().collect_vec())
        .collect_vec()
}

#[aoc(day15, part1)]
fn part_one(input: &[Vec<char>]) -> usize {
    input
        .iter()
        .map(|f| calculate_hash(&f))
        .sum()
}

#[aoc(day15, part2)]
fn part_two(input: &[Vec<char>]) -> usize {
    let mut boxes: Vec<Vec<String>> = create_vector();
    let mut hm: HashMap<(usize, String), usize> = HashMap::new();
    for i in input.iter() {
        let op = Operation::from(i);
        match op {
            Operation::Assignment(label, power) => {
                let box_id = calculate_hash(&label.chars().collect_vec());
                // println!("Setting box {} {} to {}", box_id, label, power);
                let k = (box_id, label.clone());
                if let Some(existing_lens) = hm.get_mut(&k) {
                    // If it exists only update it.
                    *existing_lens = power
                } else {
                    // If it does not exist, insert it AND make sure to update the box
                    hm.insert(k, power);
                    boxes[box_id].push(label);
                }
            },
            Operation::Remove(label) => {
                // To remove it we update the hashmap and the box
                //  but first test if its even in the hashmap to shortcut 
                //   a non entry
                let box_id = calculate_hash(&label.chars().collect_vec());
                let k = (box_id, label.clone());
                if hm.get(&k).is_some() {
                    let _ = hm.remove_entry(&k);
                    for elem in boxes[box_id].iter_mut() {
                        if *elem == label {
                            *elem = String::from("");
                        }
                    }
                }
            },
        }
    }

    let mut result: usize = 0;
    for (container_idx, container) in boxes.iter().enumerate() {
        let box_value: usize = container
            .iter()
            .filter(|s| !s.is_empty())
            .enumerate()
            .map(|(i, e)| {
                let lens_power = hm.get(&(container_idx, e.to_string())).unwrap();
                (container_idx + 1) * (i + 1) * lens_power
            })
            .sum();
        result += box_value;
    }
    result
}

fn create_vector() -> Vec<Vec<String>> {
    let cap = 256;
    let mut v: Vec<Vec<String>> = Vec::with_capacity(cap);
    for _ in 0..cap {
        v.push(vec![]);
    }
    v
}

fn hash(c: &char) -> usize {
    (c.to_ascii_lowercase() as u8) as usize
}

fn calculate_hash(f: &[char]) -> usize {
    f.iter().map(|c| hash(c)).fold(0, |acc, b| ((acc + b) * 17) % 256)
}

enum Operation {
    Assignment(String, usize),
    Remove(String),
}

impl From<&[char]> for Operation {
    fn from(value: &[char]) -> Self {
        match value[value.len() -1].is_digit(10) {
            // This is groooooooooooooooooosssss
            true => Operation::Assignment(value[0..(value.len() -2)].iter().collect(), value[value.len() -1].to_digit(10).unwrap() as usize),
            false => Operation::Remove(value[0..(value.len() -1)].iter().collect())
        }
    }
}
impl From<&Vec<char>> for Operation {
    fn from(value: &Vec<char>) -> Self {
        match value[value.len() -1].is_digit(10) {
            // This is groooooooooooooooooosssss
            true => Operation::Assignment(value[0..(value.len() -2)].iter().collect(), value[value.len() -1].to_digit(10).unwrap() as usize),
            false => Operation::Remove(value[0..(value.len() -1)].iter().collect())
        }
    }
}

impl fmt::Debug for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Assignment(a, b) => write!(f, "Set {} = {}", a, b),
            Self::Remove(a) => write!(f, "Remove {}", a),
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Assignment(a, b) => write!(f, "Set {} = {}", a, b),
            Self::Remove(a) => write!(f, "Remove {}", a),
        }
    }
}

// struct Lens<'a> {
//     label: &'a str,
//     power: u8,

// }