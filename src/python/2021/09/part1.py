# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

width = 100
all_lines = []
total = []

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

for idx, row in enumerate(all_lines):
    for idy, current in enumerate(row):
        
        if current == 0:
            total.append(current)
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
            total.append(current)

risk_level = sum(list(map(lambda x : 1 + x, total)))
print(f"The risk level is: {risk_level}")

        
        

