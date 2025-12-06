# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

EXAMPLE_MODE = True
example_answer = 0
problem_child = """[[[]]]
[[]]"""
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

def depth(L): 
    if isinstance(L, list):
        if len(L) == 1:
            l = list(map(depth, L))
            m = max(l) 
            return m +1 if m is not None else None
        elif len(L) == 0: return 1

def conv_to_word(src: list):
    # We add 97 even though ascii starts at 96 due to zero
    result = []

    # If this is ultimately an empty recurred list just toss it a letter
    z = depth(src)
    if z: 
        result.append(chr(z + 97))
    else:
        for e in src:
            if isinstance(e, list):
                result.append(conv_to_word(e))
            else:
                result.append(chr(e + 97))
    return "".join(result) if all([isinstance(x, str) for x in result]) else result

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

        res.append([conv_to_word(lhs), conv_to_word(rhs)])
    return res

def part1(src):
    res = []
    for i, line in enumerate(src):
        print(f"  - {line}")
        if sorted(line) == line:
            print(f"  adding: {line}")
            res.append(i +1)
    print(f"we got: {sum(res)}")
    print(f"{res}")

if __name__ == "__main__":
    if EXAMPLE_MODE:
        for line in example.split():
            src = example
    else:
        with open(input_file_path) as f:
            src = f.read()
    src = parse_input(src)
    part1(src)
