# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

test_input = "target area: x=20..30, y=-10..-5"

# with open(input_file_path) as f:
#     pass

