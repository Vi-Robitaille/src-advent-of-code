# Quality of life imports
from pathlib import Path
from sys import modules
from typing import final

import numpy as np

from helpers import PyGameObj as pgo

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

inp = []
with open(input_file_path) as f:
    inp = np.array([list(map(int, list(y.strip()))) for y in f.readlines()])

# I love how much cancer you can do with numpy
a = np.ones((len(inp[0]), len(inp[0])))
mod_matrix = np.vstack(([np.hstack(( [a + x for x in range(0, 5) ] )) + y for y in range(0, 5)] )) - 1

final_matrix = np.hstack(([inp] * 5))
final_matrix = np.vstack(([final_matrix] * 5))
final_matrix = final_matrix + mod_matrix
final_matrix = np.where(final_matrix >= 10, final_matrix % 10 +1, final_matrix)

search = pgo(final_matrix.flatten(), do_draw_calls=False)
search.run()

total = search.search.end.f



print(f"The path cost is {total}")
