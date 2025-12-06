import numpy as np
from scipy import ndimage

# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

steps = 2
sliding_window_footprint = (3, 3)

enhancment_key = None
inp = []

convert = lambda x: 1 if x == '#' else 0

with open(input_file_path) as f:
    enhancment_key = np.array(list(f.readline().rstrip()))
    grid = [x.rstrip() for x in f.readlines() if x != '\n']
    inp = np.array([list(map(convert, x)) for x in grid])
    inp = np.pad(inp, 1)

# Convert the given (1, 9) array to an index in the key and return the value
def get_bit_for_index(values):
    as_string = values.astype(int).astype(str)
    idx = int("".join(as_string), 2)
    result = enhancment_key[idx]
    return convert(result)

for i in range(1, steps + 1):
    print(f"Step number: {i}")
    inp = ndimage.generic_filter(np.pad(inp, 1, mode='edge'), get_bit_for_index, size=sliding_window_footprint, mode='nearest')

print(f"The resulting shape is: {inp.shape}.")
print(f"The total is: {np.sum(inp)}.")