# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "09.txt")

import numpy as np

EXAMPLE_MODE = False
example_answer = 13
example = """R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"""

def part1(input):
    tail_to_head_vec = np.array([0, 0], dtype=float)
    current_pos = np.array([0, 0], dtype=float)
    unique_places_visited = set()
    unique_places_visited.add(tuple(current_pos))
    for direction, moves in input:
        for _ in range(0, int(moves)):
            match direction:
                case "U": tail_to_head_vec[1] += 1
                case "D": tail_to_head_vec[1] -= 1
                case "L": tail_to_head_vec[0] -= 1
                case "R": tail_to_head_vec[0] += 1
            
            # If the head is 2 steps away in any direction we need to process a move
            if np.any(np.where(np.absolute(tail_to_head_vec) >= 2, True, False)):
                move = np.clip(tail_to_head_vec, -1, 1)
                current_pos += move
                unique_places_visited.add(tuple(current_pos))
                tail_to_head_vec -= move

    print(f"Visited {len(unique_places_visited)}")

if __name__ == "__main__":
    input = []

    if EXAMPLE_MODE:
        input = [x.split() for x in example.split("\n")]
    else:
        with open(input_file_path) as f:
            for line in f.readlines():
                input.append([x for x in line.strip().split(' ')])
    part1(input)
