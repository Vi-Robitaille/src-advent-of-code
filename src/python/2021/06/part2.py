# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

# This may or may not be stolen from someone smarter than i

days = 256
fishes = [0] * 9
next_day_fishes = [0] * 9

example_input = [3, 4, 3, 1, 2]
example_expected_output = 26984457539

new_fish_counter = 8
old_fish_counter = 6

with open(input_file_path) as f:
    inp = list(map(int, f.readline().split(',')))
    for i in inp:
        fishes[i] += 1


def get_next_day_reproduced_fishes():
    for i in range(new_fish_counter + 1):
        next_day_fishes[i] = 0
    for i in range(new_fish_counter + 1):
        if i == 0:
            next_day_fishes[old_fish_counter] += fishes[0]
            next_day_fishes[new_fish_counter] += fishes[0]
        else:
            next_day_fishes[i - 1] += fishes[i]


def reproduce_fishes_for_days(total_days):
    for day in range(total_days):
        get_next_day_reproduced_fishes()
        for i in range(len(fishes)):
            fishes[i] = next_day_fishes[i]


reproduce_fishes_for_days(days)
print("We got this many fish: ", sum(fishes))
