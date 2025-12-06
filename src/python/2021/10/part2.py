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
    ")": 1,
    "]": 2,
    "}": 3,
    ">": 4
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
all_scores = []
with open(input_file_path) as f:
    all_lines = [x.rstrip() for x in f.readlines()]

for line in all_lines:
    iterator = iter(line)
    if expect_next(iterator) == 0:
        total = 0
        for i in current_characters[::-1]:
            total *= 5
            total += match_values[matches[i]]
        all_scores.append(total)
    current_characters = []

sorted_values = sorted(all_scores)
middle_value = sorted_values[int(len(all_scores)/2):int(len(all_scores)/2+1)]
print(f"The total is: {middle_value}")