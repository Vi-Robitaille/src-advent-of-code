#! ../../../env/bin/python
# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "..", "input.txt")

def g(b):
    return int(b[b.find(' '):])

s, f, l, m, o = sum, filter, list, map, print
x = lambda b : g(b) if b[0] == "d" else 0
u = lambda b : -g(b) if b[0] == "u" else 0
y = lambda b : g(b) if b[0] == "f" else 0

with open(input_file_path) as f:
    i = [_.rstrip() for _ in f.readlines()]
    d = [l(m(t, i)) for t in [x, u, y]]
    o(f"The total dist is:", s(s(_) for _ in d[:-1]) * s(d[-1]))
