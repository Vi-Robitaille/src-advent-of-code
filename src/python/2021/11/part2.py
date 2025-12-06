# Quality of life imports
from pathlib import Path
from sys import modules

# OH SHIT ITS THAT BOY
import numpy as np
from functools import reduce

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

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

steps = 0
kill_loop = False

while not kill_loop:
    matrix += 1
    steps += 1
    already_triggered = np.zeros(matrix.shape, dtype=bool)
    while True:
        
        coords = [x for x in list(zip(*np.where(matrix > 9))) if not already_triggered[x[0]][x[1]]]

        if np.all(already_triggered):
            kill_loop = True
            break            

        if not len(coords):
            break

        for x in coords:
            already_triggered[x[0]][x[1]] = True

        change_matricies = [(x, create_increase_matrix(x, matrix.shape)) for x in coords]
        for coord, cm in change_matricies:
            matrix = matrix + cm
    matrix = np.where(matrix > 9, 0, matrix)

# print(matrix)
print(f"We can find a path in {steps} steps.")
print(f"We generated {num_arrays_generated} matricies.")

