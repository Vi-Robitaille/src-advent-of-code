# Quality of life imports
from pathlib import Path
from sys import modules

from helpers import PyGameObj as pgo

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

input_string = ""
with open(input_file_path) as f:
    input_string = f.read().replace('\n', '')

search = pgo(input_string, do_draw_calls=False)
search.run()

total = search.search.end.f

print(f"The path cost is {total}")
