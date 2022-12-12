use aoc_runner_derive::{aoc, aoc_generator};
use glam::IVec2;

use crate::helpies::search::{AStarSearch, Grid, MountainNode, NodeWrapper};

const START_CHAR: char = 'S';
const END_CHAR: char = 'E';

// #[aoc_generator(day12, part1)]
// fn parse_input_day12p1(input: &str) -> &AStarSearch<MountainNode> {

//     //
//     // parse here as including a parser in the lib would be annoying to handle a ton of edge cases
//     //
//     &mut generate_grid_aoc(input)
// }

// #[aoc_generator(day12, part2)]
// fn parse_input_day12p2(input: &str) -> &AStarSearch<MountainNode> {

//     //
//     // parse here as including a parser in the lib would be annoying to handle a ton of edge cases
//     //
//     &mut generate_grid_aoc(input)
// }

fn generate_grid_aoc(input: &str) -> AStarSearch<MountainNode> {
    let grid: Grid<MountainNode>;
    let mut nodes: Vec<Vec<NodeWrapper<MountainNode>>> = vec![];
    let mut start_node_position: IVec2 = IVec2 { x: 0, y: 0 };
    let mut end_node_position: IVec2 = IVec2 { x: 0, y: 0 };
    for (idx, line) in input.lines().enumerate() {
        let mut line_vec: Vec<NodeWrapper<MountainNode>> = vec![];
        for (idy, c) in line.chars().enumerate() {
            if c == START_CHAR {
                start_node_position = IVec2 {
                    x: idx as i32,
                    y: idy as i32,
                };
            }
            if c == END_CHAR {
                end_node_position = IVec2 {
                    x: idx as i32,
                    y: idy as i32,
                };
            }
            line_vec.push(NodeWrapper::new(MountainNode::new(
                idx as i32,
                idy as i32,
                (c as u32)
                    .try_into()
                    .expect("Could not unwrap this character"),
            )));
        }
        nodes.push(line_vec);
    }

    grid = Grid::new(nodes, start_node_position, end_node_position);
    AStarSearch::new(grid)
}

#[aoc(day12, part1)]
fn part_one(input: &str) -> usize {
    let mut algo = generate_grid_aoc(input);
    let mut path: Vec<NodeWrapper<MountainNode>> = vec![];
    loop {
        match algo.update() {
            Some(result) => {
                path = result;
                break;
            }
            None => continue,
        }
    }
    path.len()
}

#[aoc(day12, part2)]
fn part_two(_input: &str) -> usize {
    1
}
