
# Quality of life imports
from pathlib import Path
from sys import modules

from itertools import islice
from copy import deepcopy

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

root_transformations = {}
pairs = {}
seed = None
steps = 40

with open(input_file_path) as f:
    seed = f.readline().rstrip()
    for line in f.readlines():
        
        entry = line.rstrip()
        if entry == '':
            continue
        else:
            entry = entry.split(' -> ')
            root_transformations[entry[0]] = entry[1]
            pairs[entry[0]] = 0

for i in zip(seed, islice(list(seed), 1, None)):
    pairs["".join(i)] += 1

for _ in range(steps):
    inserts = deepcopy(pairs)
    for pair in inserts:
        count = inserts[pair]

        a = "".join([pair[0], root_transformations[pair]])
        b = "".join([root_transformations[pair], pair[1]])

        pairs[a] += count
        pairs[b] += count
        pairs[pair] -= count


results = dict([(x, 0) for x in set("".join(pairs.keys())) ])
for key in pairs.keys():
    results[key[0]] += pairs[key] / 2
    results[key[1]] += pairs[key] / 2

max_val = max(results.values())
min_val = min(results.values())
print(max_val - min_val + .5)
