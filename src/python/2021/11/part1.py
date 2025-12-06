# Quality of life imports
from pathlib import Path
from sys import modules

# OH SHIT ITS THAT BOY
import numpy as np

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

steps = 100
num_arrays_generated = 0
matrix = np.genfromtxt(input_file_path, delimiter=1)

def create_increase_matrix(index, shape):
    """
    Create a +2 oversized matrix so that a 3x3 matrix can have the correct shape for it's position
    then take the [1:-1] slice of it in both dirs.
    """

    # For the memes, how many times are we calling this?
    global num_arrays_generated
    num_arrays_generated += 1

    # Matrix representing the modification to the base matrix
    # 3x3 hollow "ones"
    a = np.ones((3, 3))
    a[1][1] = 0
    
    # now we need to pad it with zeros until a[1][1] is at the index location
    top = index[0]
    right = shape[1] - 1 - index[1]
    bottom = shape[0] - 1 - index[0]
    left = index[1]
    
    # Take the [1:-1] slice of the whole thing so that in cases like index = (0, 0)
    # We return only 3 spots with a 1 in it as we should
    return np.array([x[1:-1] for x in np.pad(a, ((top, bottom), (left, right)), "constant")[1:-1]])


light_ups = 0

for i in range(steps):
    matrix += 1
    already_triggered = np.zeros(matrix.shape, dtype=bool)
    while True:
        
        coords = [x for x in list(zip(*np.where(matrix > 9))) if not already_triggered[x[0]][x[1]]]

        if not len(coords):
            break

        for x in coords:
            already_triggered[x[0]][x[1]] = True

        change_matricies = [(x, create_increase_matrix(x, matrix.shape)) for x in coords]
        for coord, cm in change_matricies:
            matrix = matrix + cm
            light_ups += 1
    matrix = np.where(matrix > 9, 0, matrix)

# print(matrix)
print(f"There were {light_ups} light ups.")
print(f"We generated {num_arrays_generated} matricies.")
