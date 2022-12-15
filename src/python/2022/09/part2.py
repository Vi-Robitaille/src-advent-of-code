# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "09.txt")

import numpy as np

EXAMPLE_MODE = False
example_answer = 36
example = """R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"""

def part2(input):
    tail_to_head_vec = [np.array([0, 0], dtype=float) for _ in range(10)]
    unique_places_visited = set()
    unique_places_visited.add(tuple(tail_to_head_vec[-1]))
    for direction, moves in input:
        for _ in range(0, int(moves)):
            match direction:
                case "U": tail_to_head_vec[0][1] += 1
                case "D": tail_to_head_vec[0][1] -= 1
                case "L": tail_to_head_vec[0][0] -= 1
                case "R": tail_to_head_vec[0][0] += 1
            
            for i in range(0, len(tail_to_head_vec) -1):
            # If the head is 2 steps away in any direction we need to process a move
                if np.any(np.where(np.absolute(tail_to_head_vec[i] - tail_to_head_vec[i +1]) >= 2, True, False)):
                    move = np.clip(tail_to_head_vec[i] - tail_to_head_vec[i +1], -1, 1)
                    tail_to_head_vec[i+1] += move
                if i == 8:
                    unique_places_visited.add(tuple(tail_to_head_vec[-1]))

    print(f"Visited {len(unique_places_visited)}")

if __name__ == "__main__":
    input = []

    if EXAMPLE_MODE:
        input = [x.split() for x in example.split("\n")]
    else:
        with open(input_file_path) as f:
            for line in f.readlines():
                input.append([x for x in line.strip().split(' ')])
    part2(input)
