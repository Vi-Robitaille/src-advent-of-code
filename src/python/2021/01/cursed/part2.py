# Quality of life imports
from pathlib import Path
from sys import modules
from itertools import tee, islice

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "..", "input.txt")

print("The number of measurements is : ", len([a for a, b in zip(list(zip(*[islice(open(input_file_path, "r").read().split(), x, None) for x in range(3)])), list(zip(*[islice(open(input_file_path, "r").read().split(), x, None) for x in range(3)]))[1:] ) if sum(map(int, b)) > sum(map(int, a))]))
