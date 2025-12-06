# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

num_segments = {
    2: 1,
    3: 7,
    4: 4,
    # 5: [2, 5],
    # 6: [0, 3, 6, 9],
    7: 8
}

# Part 2 is for sure goign to ask to find them all. we'll do part 1 quick tho.

#  aaaa 
# b    c
# b    c
#  dddd 
# e    f
# e    f
#  gggg 

all_lines = []
total = 0

with open(input_file_path) as f:
    all_lines = [x.split('|')[1].strip() for x in f.readlines()]


for line in all_lines:
    for digit in line.split(' '):
        try:
            if num_segments[len(digit)]:
                total += 1
        except:
            continue

print(f"The total occurances of 1, 4, 7, 8 are : {total}")