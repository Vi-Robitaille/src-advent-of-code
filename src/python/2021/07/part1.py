# Quality of life imports
from statistics import median
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")


with open(input_file_path) as f:
    pos = list(map(int, f.readline().split(',')))
    target = median(pos)
    total = 0
    print(f"The median pos is: {target}")
    for i in pos:
        total += abs(target - i)
    print(f"The minimum fuel is: {total}")
