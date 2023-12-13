use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

///
/// dest source size
/// 50   98     2
///
/// 98 -> 50
/// 99 -> 51
///

/// res = value - (dest - source) if (source..source + size).contains(value) else value

#[derive(Clone)]
struct RangeMap {
    range: std::ops::Range<isize>,
    dest: isize,
    src: isize,
}

impl RangeMap {
    fn map_value(&self, value: &isize) -> Option<isize> {
        match self.range.contains(value) {
            //           50           98       50          98
            true => Some(value + (self.dest - self.src)),
            false => None,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Mappings {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl Mappings {
    fn next(&self) -> Mappings {
        match *self {
            Mappings::Seed => Mappings::Soil,
            Mappings::Soil => Mappings::Fertilizer,
            Mappings::Fertilizer => Mappings::Water,
            Mappings::Water => Mappings::Light,
            Mappings::Light => Mappings::Temperature,
            Mappings::Temperature => Mappings::Humidity,
            Mappings::Humidity => Mappings::Location,
            _ => panic!("OH NO YOU ASKED FOR A `Location`"),
        }
    }
}

impl From<&str> for Mappings {
    fn from(value: &str) -> Self {
        match value {
            "seed" => Mappings::Seed,
            "soil" => Mappings::Soil,
            "fertilizer" => Mappings::Fertilizer,
            "water" => Mappings::Water,
            "light" => Mappings::Light,
            "temperature" => Mappings::Temperature,
            "humidity" => Mappings::Humidity,
            "location" => Mappings::Location,
            &_ => panic!(),
        }
    }
}

impl ToString for Mappings {
    fn to_string(&self) -> String {
        match *self {
            Mappings::Seed => String::from("seed"),
            Mappings::Soil => String::from("soil"),
            Mappings::Fertilizer => String::from("fertilizer"),
            Mappings::Water => String::from("water"),
            Mappings::Light => String::from("light"),
            Mappings::Temperature => String::from("temperature"),
            Mappings::Humidity => String::from("humidity"),
            Mappings::Location => String::from("location"),
        }
    }
}

#[aoc_generator(day5)]
fn parse_input_day1(input: &str) -> (Vec<isize>, HashMap<Mappings, Vec<RangeMap>>) {
    let banks = input.split("\n\n").collect::<Vec<&str>>();
    let seeds = banks
        .first()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();

    let mut hm: HashMap<Mappings, Vec<RangeMap>> = HashMap::new();

    for i in banks.iter().skip(1) {
        let mut it = i.lines();

        let title = it
            .next()
            .unwrap()
            .split(' ')
            .next()
            .unwrap()
            .split('-')
            .nth(2)
            .unwrap();
        let mapping_id = Mappings::from(title);

        let mappings = it
            .map(|j| {
                let mappings = j
                    .split(' ')
                    .map(|x| x.parse::<isize>().unwrap())
                    .take(3)
                    .collect::<Vec<isize>>();

                let dest: isize = mappings[0];
                let src: isize = mappings[1];
                let size: isize = mappings[2];
                RangeMap {
                    range: (src..src + size),
                    dest,
                    src,
                }
            })
            .collect::<Vec<RangeMap>>();
        hm.insert(mapping_id, mappings);
    }

    (seeds, hm)
}

#[aoc(day5, part1)]
fn part_one(input: &(Vec<isize>, HashMap<Mappings, Vec<RangeMap>>)) -> isize {
    compare_seeds(input.0.clone().iter(), &input.1)
}

#[aoc(day5, part2)]
fn part_two(input: &(Vec<isize>, HashMap<Mappings, Vec<RangeMap>>)) -> isize {
    let smallest_seed = Arc::new(Mutex::new(isize::MAX));
    let hm = Arc::new(input.1.clone());
    println!("input.0 is {} elements long", input.0.len());
    let seed_list = input
        .0
        .windows(2)
        .step_by(2)
        .map(|x| x[0]..x[0] + x[1])
        .collect::<Vec<std::ops::Range<isize>>>();

    let num_threads = seed_list.len();

    // let (tx, rx) = channel();

    println!("Starting {} threads", num_threads);
    let mut thread_handles: Vec<JoinHandle<_>> = Vec::new();
    for g in seed_list.iter() {
        let smallest_seed = Arc::clone(&smallest_seed);
        let hm = Arc::clone(&hm);
        let g = g.clone();

        let handle = thread::spawn(move || {
            compare_seeds_thread(g, hm, smallest_seed);
        });

        thread_handles.push(handle);
    }

    for t in thread_handles {
        let _ = t.join();
    }

    let ans = *smallest_seed.lock().unwrap();
    ans
}

fn compare_seeds<'a, I>(seed_list: I, hm: &HashMap<Mappings, Vec<RangeMap>>) -> isize
where
    I: Iterator<Item = &'a isize>,
{
    let mut smallest_seed = isize::MAX;
    for seed in seed_list {
        let mut seed = *seed;
        let mut current_map = Mappings::Seed;
        loop {
            // println!("Current seed value {} in {}", seed, current_map.to_string());
            current_map = current_map.next();
            for mapper in hm.get(&current_map).unwrap() {
                if let Some(mapped_value) = mapper.map_value(&seed) {
                    // println!(" - Changing value from {} to {}", seed, mapped_value);
                    seed = mapped_value;
                    break;
                }
            }
            if current_map == Mappings::Location {
                break;
            }
        }
        // println!("Evaluating {} and {} for smallest seed", smallest_seed, seed);
        smallest_seed = isize::min(smallest_seed, seed);
        // println!("Smallest seed is now {}", smallest_seed);
    }
    smallest_seed
}

fn compare_seeds_thread(
    seed_list: std::ops::Range<isize>,
    hm: Arc<HashMap<Mappings, Vec<RangeMap>>>,
    smallest_seed: Arc<Mutex<isize>>,
) {
    // println!("Starting a thread");
    for seed in seed_list {
        // println!("Seed is: {}", seed);
        let mut seed = seed;
        let mut current_map = Mappings::Seed;
        loop {
            // println!("Current seed value {} in {}", seed, current_map.to_string());
            current_map = current_map.next();
            for mapper in hm.get(&current_map).unwrap() {
                if let Some(mapped_value) = mapper.map_value(&seed) {
                    // println!(" - Changing value from {} to {}", seed, mapped_value);
                    seed = mapped_value;
                    break;
                }
            }
            if current_map == Mappings::Location {
                break;
            }
        }
        let mut smallest_seed = smallest_seed.lock().unwrap();
        // println!("Evaluating {} and {} for smallest seed", *smallest_seed, seed);
        if isize::min(*smallest_seed, seed) == seed {
            // println!("FOUND A SMALLER NUMBER! {}", seed);
            *smallest_seed = seed;
        }
        // println!("Smallest seed is now {}", smallest_seed);
    }
}
