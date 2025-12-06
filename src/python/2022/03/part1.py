# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

example = """vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"""

example_expected_total = 157

total = 0

# Takes a character [a-zA-Z] and returns a number 1-52
# a = 1, Z = 52
def convert_ascii(c):
    char = ord(c)

    # if its upper case
    if char in range(65, 90 + 1):
        return char - 38

    # if its lowercase
    elif char in range(97, 122 + 1):
        return char - 96

    raise ValueError("Non letter character")

with open(input_file_path) as f:
    # for line in example.split('\n'):
    for line in f.readlines():
        split = int(len(line) / 2)
        lhs = line[:split]
        rhs = line[split:]
        common_element = set(lhs).intersection(rhs)

        converted = convert_ascii(common_element.pop())
        total += converted

        print(f"LHS: {lhs} || RHS: {rhs}")
        print(f"  The common element(s) is/are: {common_element}")
        print(f"  Increasing total by {converted}")

print(f"The total we got was: {total}")