# Quality of life imports
from pathlib import Path
from sys import modules
from functools import reduce

import math
import re

import itertools as it
import numpy as np


# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

SIZE_CAP = 100
KNOWN_OVERLAPS = {

}

def convert_range(input_range: str) -> list:
    spl = [list(map(int, x.split('..'))) for x in input_range]
    return spl

def convert_abs_range(input_range: list) -> int:
    return abs(input_range[0] - input_range[1])
    
def calculate_change(input_coords: list) -> int:
    return reduce(lambda x, y : x * y, input_coords)

# Unpack input into something we can use
input_lines = []
with open(input_file_path) as f:
    lines = [x.rstrip() for x in f.readlines()]
    for x in lines:
        try:

            # Yes we're unpacking all the ranges right now, in part 2 I know I will need them so wtv I can discard them now
            x_range, y_range, z_range = re.findall(r"-?\d+..-?\d+", x)
            state = re.findall(r"(on|off)", x)
            ranges = convert_range([x_range, y_range, z_range])
            if all(abs(i) <= SIZE_CAP for i in np.array(ranges).flatten()):
                input_lines.append([state[0], *ranges])

        except ValueError as e:
            print(e)


# Basically the idea is to use axis aligned 3d "bounding" boxes
# If all 3 ranges are over lapping then there is a collision
def check_overlap(current_range, current_idx, prev_elems):
    def are_overlapping(current, suspected):
        return current[1] >= suspected[0] and suspected[1] >= current[0]

    def intersect_vol(current, suspected):
        def axis_overlap(A, B):
            return max(0, min(A[1], B[1]) - max(A[0], B[0])) 
        return axis_overlap(current[0], suspected[0]) * \
               axis_overlap(current[1], suspected[1]) * \
               axis_overlap(current[2], suspected[2])

    # We need to check if the state is similar as well
    
    # What if the same region is overlapping

    # If two on segments are overlapping we need by how much they are overlapping, subtract that from the
    # New amount we're adding to the total
    
    # If two off sections are overlapping we just need

    for idx, elem in prev_elems[::-1]:
        for axis in current_range:
            if are_overlapping(axis, elem):
                # Avoid deep nesting
                continue

        # Add it to the list of known overlaps            
        if current_idx not in KNOWN_OVERLAPS:
            KNOWN_OVERLAPS[current_idx] = [idx]
        else:
            KNOWN_OVERLAPS[current_idx].append(idx)
    

# Skip copying elements into a new thing and use itertools C to do things faster as an iter
def skip_element(all_elements, idx):
    return it.chain(it.islice(all_elements, 0, idx), it.islice(all_elements, idx+1, None))


def get_previous(all_elements, idx):
    return it.chain(it.islice(all_elements, 0, idx))

ANY_OFF = False
TOTAL_ON = 0

for idx, line in enumerate(input_lines):
    state, x_range, y_range, z_range = line
    x_size = convert_abs_range(x_range)
    y_size = convert_abs_range(y_range)
    z_size = convert_abs_range(z_range)

    total_size = calculate_change([x_size, y_size, z_size])

    if state == 'off':
        ANY_OFF = True

    if not ANY_OFF and state != 'off':
        TOTAL_ON += total_size

    if ANY_OFF:
        previous_elem_iter = get_previous(input_lines, idx)
        overlap = check_overlap(line, previous_elem_iter)

        # if state == 'off':
        #     TOTAL_ON -= overlap
        # else:
        #     TOTAL_ON += overlap

print()
