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
    lines = [x.strip() for x in f.readlines()]
    for i in range(0, len(lines), 3):
        l1, l2, l3 = lines[i], lines[i + 1], lines[i + 2]

        common_element = set(l1).intersection(l2).intersection(l3).pop()

        converted = convert_ascii(common_element)
        total += converted

        # print(f"Index: {i}")
        # print(f"  The common element is: {common_element}")
        # print(f"  Increasing total by {converted}")

print(f"The total we got was: {total}")