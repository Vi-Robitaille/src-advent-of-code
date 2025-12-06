use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;
use std::collections::HashMap;
// use cached::proc_macro::cached;

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Vec<(Vec<usize>, Vec<usize>)> {
    input
        .lines()
        .map(|x| x.split(' '))
        .map(|mut x| {
            let states = new_state_string(x.next().unwrap().chars().collect_vec());
            let keys = x
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect_vec();
            (states, keys)
        })
        .collect_vec()
}

#[aoc(day12, part1)]
fn part_one(input: &[(Vec<usize>, Vec<usize>)]) -> usize {
    // let mut sum = 0;
    // let mut hm: HashMap<(usize, usize), usize> = HashMap::new();
    // for (s, k) in input {
    //     sum += count(&s, &k, &mut hm);
    //     println!("{sum}");
    // }
    // sum
    input
        .iter()
        .map(|(conditions, damaged_groups)| arrangements_count(conditions, damaged_groups))
        .sum()
}

#[aoc(day12, part2)]
fn part_two(_input: &[(Vec<usize>, Vec<usize>)]) -> u8 {
    1
}

fn new_state_string(i: Vec<char>) -> Vec<usize> {
    i.iter()
        .map(|c| match *c {
            '.' => 0,
            '?' => 1,
            '#' => 2,
            _ => panic!(),
        })
        .collect_vec()
}

// fn count(state: &[usize], keys: &[usize], hm: &mut HashMap<(usize, usize), usize>) -> usize {
//     if let Some(x) = hm.get(&(state.len(), keys.len())) {
//         return *x;
//     }
//     let total: usize = keys.iter().sum();
//     let minimum: usize = state.iter()
//         .filter(|&&x| x == 2)
//         .count();
//     let maximum: usize = state.iter()
//         .filter(|&&x| x > 0)
//         .count();

//     if minimum > total || maximum < total {
//         return 0;
//     }
//     if total == 0 {
//         return 0;
//     }
//     if let Some(f) = state.first() {
//         if *f == 0 {
//             return count(&state[1..], keys, hm);
//         } else if *f == 2 {
//             let l = keys[0];
//             let a = state[..l].iter().all(|&x| x > 0);
//             let b = state.len() == l || state[l] < 2;
//             if a && b {
//                 if state.len() == l {
//                     return 1;
//                 }
//                 return count(&state[(l + 1 )..], &keys[1..], hm);
//             }
//             return 0;
//         }
//     }
//     let mut v = vec![2];
//     v.extend_from_slice(&state[1..]);
//     count(&state[1..], keys, hm) + count(&v, keys, hm)
// }

fn arrangements(
    conditions: &[usize],
    damaged_groups: &[usize],
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(count) = cache.get(&(conditions.len(), damaged_groups.len())) {
        return *count;
    }

    let mut count = 0;

    if damaged_groups.is_empty() {
        count = if conditions.contains(&2) { 0 } else { 1 };

        cache.insert((conditions.len(), damaged_groups.len()), count);

        return count;
    }

    for offset in 0..conditions.len() {
        if conditions[0..offset].contains(&2) || offset + damaged_groups[0] > conditions.len() {
            break;
        }

        if conditions[offset..offset + damaged_groups[0]].contains(&0) {
            continue;
        }

        if damaged_groups.len() == 1 {
            if offset + damaged_groups[0] == conditions.len() {
                count += 1;
                break;
            } else {
                count += arrangements(&conditions[offset + damaged_groups[0]..], &[], cache);
                continue;
            };
        } else if offset + damaged_groups[0] + 1 > conditions.len() {
            break;
        } else if conditions[offset + damaged_groups[0]] == 2 {
            continue;
        }

        count += arrangements(
            &conditions[offset + damaged_groups[0] + 1..],
            &damaged_groups[1..],
            cache,
        );
    }

    cache.insert((conditions.len(), damaged_groups.len()), count);

    count
}

fn arrangements_count(conditions: &[usize], damaged_groups: &[usize]) -> usize {
    arrangements(conditions, damaged_groups, &mut HashMap::new())
}
