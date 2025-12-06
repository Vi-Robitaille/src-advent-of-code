# Quality of life imports
from pathlib import Path
from sys import modules
from statistics import mean

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "..", "input.txt")


with open(input_file_path.as_posix()) as f:
    strings = [list(map(float, list(x.rstrip()))) for x in f.readlines()]
    print("Gamma: {} \nEpsilon: {}".format(gamma := [round(mean(row[x] for row in strings)) for x in range(len(strings[0]))], epsilon := [1 - x for x in gamma]))
    print(int("".join(map(str,gamma)), 2) * int("".join(map(str,epsilon)), 2))

