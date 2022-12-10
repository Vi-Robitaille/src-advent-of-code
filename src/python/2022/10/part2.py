# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules["__main__"].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

EXAMPLE_MODE = False
example_answer = """##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."""
example = """addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"""


def do_cycle(cycles: int, register_x: int, result: list[int]):
    # Modifying a value IN a function like this is normally bad practice
    # THOUGH in this case we want this function to control what goes into 
    # the results, as otherwise we can make a change elsewhere and
    # not have this be the source of truth
    # OFC this is a very specific case
    # it also makes the code slightly easier to read, and this comment
    # reminds you that python doesnt copy values to functions, it only 
    # provides a name for them :)
    if cycles % 40 in range(register_x, register_x + 3):
        result.append("█")
    else:
        result.append(" ")


def part2(input):
    cycles, register_x = 1, 1
    result = []
    for line in input:
        do_cycle(cycles, register_x, result)
        cycles += 1
        match line:
            case ["noop"]:
                pass
            # Value assignment in match cases are really cool!
            case ["addx", *value]:
                do_cycle(cycles, register_x, result)
                cycles += 1
                try: # Not really needed, but its fun to add and it's good practice
                    register_x += int(*value)
                except TypeError:
                    print("The value on this is malformed, we are aexpecting this format 'opcode value'")
                    quit()

    for i in range(0, 240, 40):
        print(f'{"".join(result[i:i+39])}')


if __name__ == "__main__":
    input = []

    if EXAMPLE_MODE:
        for line in example.split("\n"):
            input.append(line.split(" "))
    else:
        with open(input_file_path) as f:
            for line in f.readlines():
                input.append(line.strip().split(" "))
    part2(input)
