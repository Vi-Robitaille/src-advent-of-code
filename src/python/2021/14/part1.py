
#
# This code is beautiful. you dont know what you're talking about.
#

# Quality of life imports
from pathlib import Path
from sys import modules

from collections import deque, Counter

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "Input_Challenge_14.txt")
solution_file_path = Path(src, "solution-kev.txt")

root_transformations = {}
seed = None
mutable_list = deque()
operations = 0
total_ops = []
steps = 10


with open(input_file_path) as f:
    seed = deque(f.readline().rstrip())
    print(len(seed))
    for line in f.readlines():
        
        entry = line.rstrip()
        if entry == '':
            continue
        else:
            entry = entry.split(' -> ')
            root_transformations[(entry[0][0], entry[0][1])] = entry[1]
    
    
print(f"{0} steps, {operations} ops, biggness: {len(seed)}")

result = deque()
for i in range(steps):
    operations = 0
    mutable_list = deque()
    for idx, element in enumerate(seed):
        mutable_list.append(element)
        if idx +1 == len(seed):
            break
        pair = (element, seed[idx +1])
        if pair in root_transformations:
            operations += 1
            insert = root_transformations[pair]
            mutable_list.append(insert)
    seed = deque()
    for _ in mutable_list: seed.append(_)
    # print(f"{i+1} steps, {operations} ops, biggness: {len(mutable_list)}")

# with open(solution_file_path, "w") as f:
#     f.write("".join(mutable_list))

totals = Counter(mutable_list)
occurences = totals.most_common(None)
largest_occurence = occurences[0][1]
smallest_occurence = occurences[-1][1]
print(f"The difference is {largest_occurence - smallest_occurence}")
