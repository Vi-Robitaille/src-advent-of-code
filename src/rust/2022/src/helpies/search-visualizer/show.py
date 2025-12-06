# Quality of life imports
from pathlib import Path
from sys import modules

DEBUG = False

# Quality of life, define the input file location
# stolen from my python aoc
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "out.txt") if DEBUG else Path(src, "out-real.txt")
grid_file_path = Path(src, "grid.txt")

from PIL import Image
import numpy as np
from dataclasses import dataclass
from math import floor

example_grid = """Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"""

START_COLOR = np.array([255, 172, 28], dtype=float)
END_COLOR = np.array([250, 128, 114], dtype=float)
DIFFERENCE_COLOR = END_COLOR - START_COLOR
SCREEN_MULT = 8
GRID_DOT_SIZE = floor(SCREEN_MULT / 2) if SCREEN_MULT % 2 != 0 else SCREEN_MULT / 2 -1

@dataclass
class Point:
    x: int
    y: int

    def __init__(self, x, y):
        self.x = x +1
        self.y = y +1

    def __eq__(self, __o: object) -> bool:
        if isinstance(__o, Point):
            return self.x == __o.x and self.y == __o.y
        raise TypeError(f"Cannot compare type Point and {type(__o)}.")

def grid_color(slot_height: str):
    if len(slot_height) != 1:
        raise ValueError
    # return np.array([ord(slot_height)] *3)
    val = ((ord(slot_height) - 97) / (122 - 97)) * 255
    return np.array([val] *3)

def path_color(path_length: int, node_index: int):
    return START_COLOR + (DIFFERENCE_COLOR / path_length) * node_index

def load_grid() -> list[list[str]]:
    if DEBUG:
        grid_string = example_grid
    else:
        with open(grid_file_path) as f: 
            grid_string = f.read()
    return [list(x) for x in grid_string.split('\n') if x != ""]

def get_path_points() -> list[Point]:
    points = []
    with open(input_file_path, "r") as file_handle:
        for line in file_handle:
            stripped = line.strip()
            if stripped == "": continue
            x, y, *_ = stripped.split(',')
            points.append(Point(int(x.split(' ')[-1]), int(y.split(' ')[-1])))
    # if DEBUG:
        # print(f"Got these points")
        # [print(x) for x in points]
    return points


def generate_circle():
    # xx and yy are 200x200 tables containing the x and y coordinates as values
    # mgrid is a mesh creation helper
    xx, yy = np.mgrid[:SCREEN_MULT-1, :SCREEN_MULT-1]
    # circles contains the squared distance to the (100, 100) point
    # we are just using the circle equation learnt at school
    circle = (xx - GRID_DOT_SIZE) ** 2 + (yy - GRID_DOT_SIZE) ** 2
    where = 13
    return circle< where

def main():
    grid = load_grid()
    image_size = (len(grid) * 8 +1, len(grid[0]) * 8+1, 3)
    path = get_path_points()

    image = np.zeros(image_size, dtype=np.uint8)
    print(f"image_size: {image.shape}")
    circle = generate_circle()
    for index_x, element_x in enumerate(grid):
        for index_y, element_y in enumerate(element_x):
            spot = Point(index_x, index_y)
            if spot in path:
                color = path_color(len(path), path.index(spot))
            else:
                color = grid_color(element_y)

            for idx, ex in np.ndenumerate(circle):
                if ex:
                    x = 1+index_x*SCREEN_MULT+idx[0]
                    y = 1+index_y*SCREEN_MULT+idx[1]
                    image[x][y][0] = color[0]
                    image[x][y][1] = color[1]
                    image[x][y][2] = color[2]
            
    im = Image.fromarray(image)
    im.save(f"{src}/ouput.png")

if __name__ == "__main__":
    main()

