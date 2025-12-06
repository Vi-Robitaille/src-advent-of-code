# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

num_segments = {
    2: 1,
    3: 7,
    4: 4,
    5: [2, 3, 5],
    6: [0, 6, 9],
    7: 8
}

#  dddd
# e    a
# e    a
#  ffff
# g    b
# g    b
#  cccc

# Truth table
# 0 : a b c d e   g
# 1 : a b
# 2 : a   c d   f g
# 3 : a b c d   f
# 4 : a b     e f
# 5 :   b c d e f
# 6 :   b c d e f g
# 7 : a b   d
# 8 : a b c d e f g
# 9 : a b c d e f

# Non 1, 4, 7, 8 numbers
# 0 is the only one missing f
# 2 is the only one missing b
# 3 is the only one of len 6 missing e, we isolate the e by checking for the one 
# 5 is the same as 6 minus g, we can sort them and check, if the first 5 elements are the same its good
# 9 is the only one of len 6 missing the element from the 5-6 comp

def num_common_elements(list_a, list_b):
    if len(list_a) != len(list_b):
        return 0
    return sum(x == y for x, y in zip(list_a, list_b))

def difference(list_a, list_b, override=False):
    if len(list_a) > len(list_b) or override:
        return list(set(list_a) - set(list_b))
    else:
        return list(set(list_b) - set(list_a))

total = 0
all_lines = []
with open(input_file_path) as f:
    all_lines = [x.rstrip() for x in f.readlines()]

for line in all_lines:

    key, value = line.split('|')
    
    # 2, 3, 5
    five_segments = [list(x) for x in key.split() if len(x) == 5]
    
    # 0, 6, 9
    six_segments = [list(x) for x in key.split() if len(x) == 6]

    # We sort all elements so we have an easier time later
    numbers = {
        "1": list([x for x in key.split() if len(x) == 2][0]),
        "4": list([x for x in key.split() if len(x) == 4][0]),
        "7": list([x for x in key.split() if len(x) == 3][0]),
        "8": list([x for x in key.split() if len(x) == 7][0])
    }

    # Three is the only one thats going to knock out both of the segments in one
    numbers["3"] = [x for x in five_segments if len(set(numbers["1"]) - set(x)) == 0][0]
    
    # Remove three so we dont need to consider it again
    five_segments.remove(numbers["3"])

    bottom_left_segment = min(set(numbers["8"]) - set(numbers["3"] + numbers["4"]))

    # If we add 3 and 4 together we get all segments that are not the bottom left 
    numbers["5"] = [x for x in five_segments if bottom_left_segment not in x][0]
    
    # Remove 5
    five_segments.remove(numbers["5"])
    
    # Two is the last one left!
    numbers["2"] = five_segments[0]

    # Now we need to find 0, 6, and 9
    # 9 is easy, subtract the bottom left segment and we've found it if the len doesnt change
    numbers["9"] = [x for x in six_segments if bottom_left_segment not in x][0]

    six_segments.remove(numbers["9"])

    # For 6 we can find it by subtracting the set of 5 from the set of 4, that should leave us with the top right segment
    # Due to the weirdness of sets we need to user min or max to get the element out
    top_right_segment = min(set(numbers["4"]) - set(numbers["5"]))
    numbers["6"] = [x for x in six_segments if top_right_segment not in x][0]

    # Remove 6 so we're only left with 0
    six_segments.remove(numbers["6"])

    numbers["0"] = six_segments[0]

    # OK, We have all the numbers now, time to print out the values

    number = []
    numbers_keys = list(numbers.keys())
    numbers_values = [sorted(x) for x in list(numbers.values())]
    for digit in value.split():
        pos = numbers_values.index(sorted(digit))
        number.append(numbers_keys[pos])
    
    total += int("".join(number))

print(total)

        
        
    


        






