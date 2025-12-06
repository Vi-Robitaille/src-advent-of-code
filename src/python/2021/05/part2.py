# Quality of life imports
from pathlib import Path
from itertools import cycle
import sys

# Quality of life, define the input file location
src = Path(sys.modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

# Import helper functions
helper_location = Path(src, "..", "..", "helpers")
sys.path.insert(1, helper_location.as_posix())

from helpies import range_inclusive
from peekiter import PeekIter


def check_spot(x, y, board):
    if (x, y) in board:
        # If its in the board already its an overlap
        board[(x, y)] = 1
    else:
        board[(x, y)] = 0


def generate_points(x1, y1, x2, y2, board):

    # You dont need to read this :)
    if x1 == x2:
        x = PeekIter(cycle([x1]))
        y = PeekIter(range_inclusive(*sorted([y1, y2])))
    elif y1 == y2:
        x = PeekIter(range_inclusive(*sorted([x1, x2])))
        y = PeekIter(cycle([y1]))
    else:
        x = PeekIter(range_inclusive(x1, x2)) if x1 < x2 else PeekIter(
            range_inclusive(x1, x2, -1))
        y = PeekIter(range_inclusive(y1, y2)) if y1 < y2 else PeekIter(
            range_inclusive(y1, y2, -1))

    while (x.has_next() and y.has_next()):
        check_spot(next(x), next(y), board)


input, board = [], {}
with open(input_file_path) as f:
    while (inp := f.readline().rstrip()) != "":
        pos = list(map(int, inp.replace(' -> ', ',').split(',')))
        generate_points(pos[0], pos[1], pos[2], pos[3], board)

print(sum(board.values()))
