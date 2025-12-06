# Quality of life imports
from pathlib import Path
from sys import modules
from itertools import tee, islice

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "..", "input.txt")

print("The number of measurements is : ", len([a for a, b in list(zip(*[islice(open(input_file_path, "r").read().split(), x, None) for x in range(2)])) if int(b) > int(a)]))


def foo(a: list[float | int] = [2.]):
    return a.append(1.)

