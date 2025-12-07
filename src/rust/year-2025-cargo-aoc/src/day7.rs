use ordermap::OrderMap;
use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Inp = (usize, OrderMap<usize, Vec<usize>>);

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Inp {
    let start = input.chars().position(|c| c == 'S').unwrap();

    let mut hm: OrderMap<usize, Vec<usize>> = OrderMap::new();
    for (i, e) in input.lines().enumerate() {
        let indexes = e
            .char_indices()
            .filter_map(|(c_i, c_e)| (c_e == '^').then_some(c_i))
            .collect_vec();
        if !indexes.is_empty() {
            hm.insert(i, indexes);
        }
    }

    (start, hm)
}

#[aoc(day7, part1)]
fn part_one(input: &Inp) -> usize {
    let mut beam_splits = 0;
    let mut beam_indexes: HashSet<usize, _> = {
        let mut _hs = HashSet::new();
        _hs.insert(input.0);
        _hs
    };

    // slide down the sequence
    for (_, reflector_spots) in &input.1 {
        // if this horizontal line contains a splitter at a current beam index
        let mut tmp_beam_indexes = HashSet::new();
        for beam in &beam_indexes {
            if reflector_spots.contains(beam) {
                // "split" the beam and increment `beam_splits`
                tmp_beam_indexes.insert(*beam - 1);
                tmp_beam_indexes.insert(*beam + 1);
                beam_splits += 1;
            } else {
                tmp_beam_indexes.insert(*beam);
            }
        }
        beam_indexes = tmp_beam_indexes;
    }
    beam_splits
}

#[aoc(day7, part2)]
fn part_two(input: &Inp) -> usize {
    // seems there's an off by one error somewhere here, i dont care enough to find it
    let mut beam_splits = 1;
    let mut beam_indexes: HashMap<usize, usize> = {
        let mut hm = HashMap::new();
        hm.insert(input.0, 1);
        hm
    };

    // slide down the sequence
    for (row, reflector_spots) in &input.1 {
        println!("Current row: {}", row);
        // if this horizontal line contains a splitter at a current beam index
        let mut tmp_beam_indexes = HashMap::new();
        for (beam, count) in &beam_indexes {
            if reflector_spots.contains(beam) {
                // "split" the beam and increment `beam_splits`
                tmp_beam_indexes
                    .entry(*beam - 1)
                    .and_modify(|e| *e += count)
                    .or_insert(*count);
                tmp_beam_indexes
                    .entry(*beam + 1)
                    .and_modify(|e| *e += count)
                    .or_insert(*count);
                beam_splits += count;
            } else {
                tmp_beam_indexes
                    .entry(*beam)
                    .and_modify(|e| *e += count)
                    .or_insert(*count);
            }
        }
        beam_indexes = tmp_beam_indexes;
    }
    beam_splits
}
