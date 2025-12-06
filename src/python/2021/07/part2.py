# Quality of life imports
from pathlib import Path
from statistics import mean
import sys

# Quality of life, define the input file location
src = Path(sys.modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

# Import helper functions
helper_location = Path(src, "..", "..", "helpers")
sys.path.insert(1, helper_location.as_posix())

from helpies import range_inclusive


with open(input_file_path) as f:
    pos = list(map(int, f.readline().split(',')))
    answers = []

    # I've learned for some inputs you need to round up
    targets = [int(mean(pos)), int(mean(pos)) + 1]
    for target in targets:
        print(f"The target pos is: {target}")
        total = 0
        for i in pos:
            dist = abs(target - i)
            area = sum(range_inclusive(1, dist))
            total += area
        answers.append([target, total])
    minimum = sorted(answers, key=lambda x : x[1])[0]
    print(f"The smallest amount is at pos: {minimum[0]}\n\t value: {minimum[0]}")
