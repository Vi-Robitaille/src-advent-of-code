# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

import numpy as np
import numpy.typing as npt
import re

# I just found this
# I love this library
from shapely.ops import unary_union, clip_by_rect
from shapely.geometry import Polygon

EXAMPLE_MODE = False
example_answer = 0
example = """Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"""

point_regex = re.compile(r'-?\d+')

def part2(src):
    poly = Polygon()
    for sensor_x, sensor_y, beacon_x, beacon_y in parse_input(src):
        poly = intersect(poly, sensor_x, sensor_y, beacon_x, beacon_y)
    sol = clip_by_rect(poly, 0, 0, 4_000_000, 4_000_000).interiors[0]
    x, y = map(round, sol.centroid.coords[:][0])
    print(f"Found at x: {x}, y: {y} - {x * 4_000_000 + y}")

def parse_input(src):
    for line in src:
        try:
            sensor_x, sensor_y, beacon_x, beacon_y = [int(x) for x in point_regex.findall(line)]
            if EXAMPLE_MODE:
                print(f"Sensor at x={sensor_x}, y={sensor_y}: closest beacon is at x={beacon_x}, y={beacon_y}")
            yield sensor_x, sensor_y, beacon_x, beacon_y
        except ValueError:
            print(f"Found too many number sets on this line: '{line}'")
            quit()

def taxicab_dist(a: npt.ArrayLike, b: npt.ArrayLike) -> int:
    return np.abs(a - b).sum()

def intersect(poly, sensor_x, sensor_y, beacon_x, beacon_y):
    """hoho i KNEW this could be done via GEOMETRY"""
    dist = taxicab_dist(np.array([sensor_x, sensor_y]), np.array([beacon_x, beacon_y]))
    return unary_union([
        poly, Polygon(
            [(sensor_x, sensor_y + dist), (sensor_x - dist, sensor_y), (sensor_x, sensor_y - dist), (sensor_x + dist, sensor_y)]
        )
    ])

if __name__ == "__main__":
    if EXAMPLE_MODE:
            src = example.split("\n")
    else:
        with open(input_file_path) as f:
            src = f.readlines()
    part2(src)

