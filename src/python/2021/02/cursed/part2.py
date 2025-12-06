# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "..", "input.txt")


def main():
    x, y, a = 0, 0, 0
    with open(input_file_path) as f:
        instructions = f.readlines()
        for instruction in instructions:
            s = instruction.rstrip().split()
            command = s[0]
            dist = int(s[1])
            if command == "forward":
                x += dist
                y += (a * dist)
            elif command == "down":
                a += dist
            elif command == "up":
                a -= dist
    print(f"The total dist is {x * y}")


if __name__ == "__main__":
    main()
