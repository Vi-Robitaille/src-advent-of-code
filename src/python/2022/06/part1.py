# Quality of life imports
from pathlib import Path
from sys import modules

import time
import unittest

# Quality of life, define the input file location
src = Path(modules["__main__"].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

DEBUG = False
offset = 4


def solution(input):
    for i, _ in enumerate(input):
        if DEBUG:
            print("".join(input[i : i + offset]))
        if len(set(input[i : i + offset])) == offset:
            return i + offset


class UnitTest(unittest.TestCase):
    def test_one(self):
        self.assertEqual(solution("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5)

    def test_two(self):
        self.assertEqual(solution("nppdvjthqldpwncqszvftbrmjlhg"), 6)

    def test_three(self):
        self.assertEqual(solution("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10)

    def test_four(self):
        self.assertEqual(solution("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11)


if __name__ == "__main__":
    with open(input_file_path) as f:
        if not DEBUG:
            start = time.time()
        result = solution([x for x in f.readline().strip()])
        if not DEBUG:
            print(f"Time elapsed: {time.time() - start:.6f} seconds")
        print(f"Found at: {result}")
