# Quality of life imports
from pathlib import Path
from sys import modules
from statistics import mean
from typing import Tuple
from copy import deepcopy

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "..", "input.txt")


def sum_index(array, index):
    if (m := mean([x[index] for x in array])) == 0.5:
        return 1.0
    return round(m)


def get_with_same_bit(match, array, index):
    return [x for x in array if x[index] == match]    


with open(input_file_path.as_posix()) as f:
    strings = [list(map(float, list(x.rstrip()))) for x in f.readlines()]
    strings_two = deepcopy(strings)

    for i in range(len(strings[0])):
        if not len(strings) == 1:
            strings = get_with_same_bit(sum_index(strings, i), strings, i)
        if not len(strings_two) == 1:
            strings_two = get_with_same_bit(1 - sum_index(strings_two, i), strings_two, i)
        
    print(a := int("".join(map(str, [int(x) for x in strings[0]])), 2))
    print(b := int("".join(map(str, [int(x) for x in strings_two[0]])), 2))
    print(a * b)
