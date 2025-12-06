# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

# from itertools import zip_longest

EXAMPLE_MODE = False
example_answer = 0
example = """[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"""

def parse_input(src: str):
    res = []
    from yaml import load
    try:
        from yaml import CLoader as Loader
    except ImportError:
        from yaml import Loader
    for pair in src.split('\n\n'):
        spl = pair.split('\n')
        lhs = load(spl[0], Loader=Loader)
        rhs = load(spl[1], Loader=Loader)

        res.append([lhs, rhs])
    return res


def cmp(lhs, rhs) -> int:
    for i in range(0, len(lhs)):
        if i >= len(rhs):
            return -1
        a, b, x = lhs[i], rhs[i], 0
        # MATCH STATEMENTS ARE FUN LEAVE ME ALONE
        match (isinstance(a, int), isinstance(b, int)):
            case True, True:
                if a > b: return -1
                elif a < b: return 1
            case False, True:
                x = cmp(a, [b])
            case True, False:
                x = cmp([a], b)
            case False, False:
                x = cmp(a, b)
        if x != 0: return x
    if len(lhs) == len(rhs): return 0
    else: return 1


def part1(src):
    l = [i+1 for i, elem in enumerate(src) if cmp(*elem) > 0]
    result = sum(l)
    print(f"Part 1: {result}")

if __name__ == "__main__":
    if EXAMPLE_MODE:
        for line in example.split():
            src = example
    else:
        with open(input_file_path) as f:
            src = f.read()
    src = parse_input(src)
    part1(src)
