# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

import numpy as np
import numpy.typing as npt
import re
import cProfile

EXAMPLE_MODE = False
example_answer = 0
# example = """Sensor at x=8, y=7: closest beacon is at x=2, y=10"""
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

def part1(src):
    """
    Calcualte the used cells by summing the lengths of the 1d bounding boxes
      we could also go for a much more memory heavy solution of adding 
        the range(sensor_x - slots_used, sensor_x + slots_used +1) to a set
          and counting the resulting elements
    """
    y = 10 if EXAMPLE_MODE else 2000000
    unavailable_locations = []
    for sensor, beacon in parse_input(src):
        bounding_box = aabb_at_coordinate(sensor, beacon, y)
        if bounding_box is not None:
            evaluate_aabb(unavailable_locations, bounding_box)
    print(f"{[taxicab_dist(*x[:,[0]]) for x in unavailable_locations]}")

def parse_input(src):
    for line in src:
        try:
            sensor_x, sensor_y, beacon_x, beacon_y = [int(x) for x in point_regex.findall(line)]
            if EXAMPLE_MODE:
                print(f"Sensor at x={sensor_x}, y={sensor_y}: closest beacon is at x={beacon_x}, y={beacon_y}")
            yield np.array([sensor_x, sensor_y]), np.array([beacon_x, beacon_y])
        except ValueError:
            print(f"Found too many number sets on this line: '{line}'")
            quit()

def taxicab_dist(a: npt.ArrayLike, b: npt.ArrayLike) -> int:
    return np.abs(a - b).sum()

def dist_to_y(a: npt.ArrayLike, y) -> int:
    return taxicab_dist(np.array([0, y]), np.array([0, a[1]]))

def aabb_at_coordinate(sensor: npt.ArrayLike, beacon: npt.ArrayLike, coordinate: int) -> npt.ArrayLike:
    """Returns an "axis aligned bounding box" to define the 1d shape that is the cells used for this sensor/beacon combo
        
    :rtype: numpy.array
    :return: A 2x2 numpy array, x is the range of cells, y = 0
    """
    total_search_range = taxicab_dist(sensor, beacon)
    slots_used = total_search_range - dist_to_y(sensor, coordinate)
    if slots_used > 0:
        start, end = sensor[0] - slots_used, sensor[0] + slots_used
        return np.array([[start, 0], [end, 0]])

def aabb(lhs: npt.ArrayLike, rhs: npt.ArrayLike):
    """ We can use "axis aligned bounding boxes" to compare if these 1d shapes are overlapping
    If they are we then extend the existing one, if not add the new one
    
    :rtype: None or new numpy.array
    :return: new array encapsulating both bounding boxes
    """
    x1 = lhs[:,[0]].flatten()
    x1.sort()
    x2 = rhs[:,[0]].flatten()
    x2.sort()
    
    v1, v2 = x1
    v3, v4 = x2

    if v1 in range(v3, v4) and v2 not in range(v3, v4):
        return np.array([[v3, rhs[0][1]],[v2, lhs[1][1]]])
    elif v2 in range(v3, v4) and v1 not in range(v3, v4):
        return np.array([[v1, lhs[0][1]],[v4, rhs[1][1]]])
    return None

def evaluate_aabb(bounding_boxes: list[npt.ArrayLike], new_bounding_box: npt.ArrayLike):
    """
    Evaluate all bounding boxes, if the provided one intersects one of them, extend that one
    we then need to evaluate all of them again with this new extended one
    
    """
    for i, e in enumerate(bounding_boxes):
        r = aabb(e, new_bounding_box)
        if r is not None:
            bounding_boxes.pop(i)
            evaluate_aabb(bounding_boxes, r)
            return
    bounding_boxes.append(new_bounding_box)


if __name__ == "__main__":
    if EXAMPLE_MODE:
            src = example.split("\n")
    else:
        with open(input_file_path) as f:
            src = f.readlines()
    cProfile.run('part1(src)')
    # part1(src)
    # print(timeit.timeit("part1(src)", globals=locals(), number=10))
    

# Calc dist from each point to y=2000000
# Unavail slots are 1 + (2 * (dist_to_closest_beacon - dist_to_y ))

#                1    1    2    2
#      0    5    0    5    0    5
# -2 ..........#.................
# -1 .........###................
#  0 ....S...#####...............
#  1 .......#######........S.....
#  2 ......#########S............
#  3 .....###########SB..........
#  4 ....#############...........
#  5 ...###############..........
#  6 ..#################.........
#  7 .#########S#######S#........ width here is 19 
#  8 ..#################.........  total taxicab dist is 9 to beacon
#  9 ...###############..........
# 10 ....B############........... dist to here is 3
# 11 ..S..###########............  values are (x +/- (9 - 3), 10)
# 12 ......#########.............
# 13 .......#######..............
# 14 ........#####.S.......S.....
# 15 B........###................
# 16 ..........#SB...............
# 17 ................S..........B
# 18 ....S.......................
# 19 ............................
# 20 ............S......S........
# 21 ............................
# 22 .......................B....