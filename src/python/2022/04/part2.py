# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

example = """2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"""
example_answer = 2

total = 0

with open(input_file_path) as f:
    for line in f.readlines():
    # for line in example.split('\n'):

        # leave me alone i have headache
        # my copy paste code seems to have been a great fit for part 2 lmao
        # time for some advil...
        container = line.strip().split(',')
        lhs = container[0].split('-')
        rhs = container[1].split('-')

        lhs = range(int(lhs[0]), int(lhs[1]) + 1)
        rhs = range(int(rhs[0]), int(rhs[1]) + 1)
        
        if len(set(lhs).intersection(rhs)) > 0:
            total += 1

print(f"The total is: {total}")

