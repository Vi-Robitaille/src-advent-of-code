# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

inp = []
with open(input_file_path) as f:
    # print(len(f.readline().split(',')))
    inp = list(map(int, f.readline().split(',')))

example_input = [3, 4, 3, 1, 2]
example_expected_output = 5934

days = 80
total = len(inp)


def spawn_kids(spawn_date):
    global total
    for i in range(spawn_date, 0, -7):
        if i < 0:
            return
        total += 1
        spawn_kids(i - 9)


for number in inp:
    spawn_date = days - number
    if spawn_date < 0:
        continue
    spawn_kids(spawn_date)

print(total)
