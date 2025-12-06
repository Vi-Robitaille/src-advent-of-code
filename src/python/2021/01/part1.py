# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")


def main():

    # Open the input file and get a handle
    with open(input_file_path.as_posix()) as f:

        # Create a storage location for the result and the previous number 
        # the value of the line should never exceed the default so we will not hit an
        # off by one error
        accumulator, previous = 0, 999999999

        # Iterate over each line
        for line in f.readlines():
            if int(line) > previous:
                accumulator += 1
            previous = int(line)
        print(f"The number of measurements is : {accumulator}")


if __name__ == "__main__":
    main()
