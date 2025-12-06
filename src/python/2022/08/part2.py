# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules["__main__"].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

from math import prod
import numpy as np


EXAMPLE_MODE = False
example = """30373
25512
65332
33549
35390"""
example_answer = 21

def number_visible(arr: list[int], cap: int = 10) -> list[int]:
    return next((d for d, tree in enumerate(arr, start=1) if tree >= cap), len(arr))


def part2(input):
    forest = np.array(input, dtype=int)
    transpose_forest = np.transpose(np.array(input, dtype=int))

    max_score = 0
    for i, x in enumerate(forest):
        for j, y in enumerate(transpose_forest):
            trees = (y[:i][::-1], y[i + 1 :], x[:j][::-1], x[j + 1 :])
            score = prod([number_visible(arr=x, cap=y[i]) for x in trees])
            max_score = max(score, max_score)
    print(f"The most bestest spot has a score of: {max_score}")


if __name__ == "__main__":
    input = []

    if EXAMPLE_MODE:
        for line in example.split("\n"):
            input.append([int(x) for x in line])
    else:
        with open(input_file_path) as f:
            for line in f.readlines():
                input.append([int(x) for x in line.strip()])
    part2(input)
