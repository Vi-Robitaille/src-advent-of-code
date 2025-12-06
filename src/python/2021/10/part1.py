# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")


# To be used as FILO
current_characters = []

def peek():
    if len(current_characters) > 0:
        return current_characters[-1]
    else:
        raise IndexError

matches = {
    "(": ")",
    "[": "]",
    "{": "}",
    "<": ">"
}
match_values = {
    ")": 3,
    "]": 57,
    "}": 1197,
    ">": 25137
}

def expect_next(iterator):
    try:
        next_val = next(iterator)
    except StopIteration:
        return 0
    
    # This is an opening bracket so append it to the 
    if next_val in matches.keys():
        current_characters.append(next_val)

    if matches[peek()] == next_val:
        current_characters.pop()
        return

    if matches[peek()] != next_val and next_val in match_values.keys():
        return match_values[next_val]
    
    while True:
        return_val = expect_next(iterator)
        if isinstance(return_val, int):
            return return_val



all_lines = []
with open(input_file_path) as f:
    all_lines = [x.rstrip() for x in f.readlines()]

total = 0
for line in all_lines:
    iterator = iter(line)
    total += expect_next(iterator)

print(f"The total is: {total}")