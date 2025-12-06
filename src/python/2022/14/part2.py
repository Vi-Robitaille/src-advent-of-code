# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

EXAMPLE_MODE = True
example_answer = 0
example = """"""

def part2(input):
    pass

if __name__ == "__main__":
    input = []

    if EXAMPLE_MODE:
        for line in example.split("\n"):
            pass
    else:
        with open(input_file_path) as f:
            for line in f.readlines():
                pass
    part2(input)
