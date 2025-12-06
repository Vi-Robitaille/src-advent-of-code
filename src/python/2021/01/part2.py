# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")


def main():
    
    # Create a storage location for the result and the previous number 
    # the value of the line should never exceed the default so we will not hit an
    # off by one error
    accumulator, previous = 0, 999999999
    block_size = 3

    # Open the input file and process the lines into a more easily usable format
    with open(input_file_path.as_posix()) as f:
        lines = [int(x.rstrip()) for x in f.readlines()]

    # Get the number of 3 block sized elements we can iterate over
    # without a trailing undersized block
    iters = len(lines) - 2
    
    # Iterate over the entries from the file, calculate the size of the 3 line block
    # and evaluate it against the previous 
    for i in range(iters):
        total = sum(lines[i: i + block_size])
        if total > previous:
            accumulator += 1
        previous = total

    print(f"The number of measurements is : {accumulator}")

            
if __name__ == "__main__":
    main()
