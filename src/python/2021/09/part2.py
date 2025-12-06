# Quality of life imports
from functools import reduce
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

width = 100
all_lines = []
basin_roots = []
all_sizes = []

def get_with_index(lines, idx, idy):
    if idx == -1 or idy == -1:
        return -1
    # I might as well use GoTos
    try:
        return lines[idx][idy]
    except IndexError:
        return -1

with open(input_file_path) as f:
    lines = list(map(list, [y.strip() for y in f.readlines()]))
    all_lines = [list(map(int, x)) for x in lines]
    width = len(all_lines[0])

for idx, row in enumerate(all_lines):
    for idy, current in enumerate(row):
        coords = (idx, idy)
        if current == 0:
            basin_roots.append(coords)
            continue
        if current == 9:
            continue

        # Assign indexes to be used 
        up = get_with_index(all_lines, idx - 1, idy)
        down = get_with_index(all_lines, idx + 1, idy)
        left = get_with_index(all_lines, idx, idy - 1)
        right = get_with_index(all_lines, idx, idy + 1)
        
        neighbors = [x for x in [up, down, left, right] if x != -1]
        if current < min(neighbors):
            basin_roots.append(coords)

# Now that we have basin roots, flood fill each one to find the largest!

def flood_fill(matrix, coords):
    idx, idy = coords
    size = 0

    current = matrix[idx][idy]

    # Just make super sure
    if current == 9 or current == -1:
        return 0

    if current != -1:
        matrix[idx][idy] = -1
        size += 1

    up = (get_with_index(all_lines, idx - 1, idy), (idx - 1, idy))
    down = (get_with_index(all_lines, idx + 1, idy), (idx + 1, idy))
    left = (get_with_index(all_lines, idx, idy - 1), (idx, idy - 1))
    right = (get_with_index(all_lines, idx, idy + 1), (idx, idy + 1))
    neighbors = [x for x in [up, down, left, right] if x[0] != -1 and x[0] != 9]

    if len(neighbors) == 0:
        return size

    for n in neighbors:
        size += flood_fill(matrix, n[1])
        
    return size


for root in basin_roots:
    all_sizes.append(flood_fill(all_lines, root))

top_values = sorted(all_sizes, reverse=True)[:3]
question_answer = reduce(lambda x, y: x * y, top_values)
print(question_answer)
