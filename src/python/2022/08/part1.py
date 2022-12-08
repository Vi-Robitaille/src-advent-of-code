# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules["__main__"].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

import numpy as np

EXAMPLE_MODE = False
example = """30373
25512
65332
33549
35390"""
example_answer = 21

def is_visible(tree: int, arr: list[list[int]]) -> bool:
    return any(tree > max(x, default=-1) for x in arr)


def part1(input: list[list[int]]):
    visible = 0
    forest = np.array(input, dtype=int)
    transpose_forest = np.transpose(np.array(input, dtype=int))

    for idx, row in enumerate(forest):
        for idy, col in enumerate(transpose_forest):

            tree = col[idx]
            trees = (col[:idx][::-1], col[idx + 1 :], row[:idy][::-1], row[idy + 1 :])

            visible += is_visible(tree, trees)

    print(f"We found {visible} trees")


if __name__ == "__main__":
    input = []

    if EXAMPLE_MODE:
        for line in example.split("\n"):
            input.append([int(x) for x in line])
    else:
        with open(input_file_path) as f:
            for line in f.readlines():
                input.append([int(x) for x in line.strip()])
    part1(input)
