# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")


example = """1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"""


with open(input_file_path) as f:
    elves = f.read().split('\n\n')
    # elves = example.split('\n\n')
    totals = [sum([int(y.strip()) for y in x.split('\n')]) for x in elves]
    print(f"The max is {max(totals)}")

