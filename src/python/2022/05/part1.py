# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "aoc_2022_day05_large_input.txt")

import re

example = """    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"""

example_answer = "CMZ"

crate_regex = re.compile(r'(\[\D\]|\s{3})\s?')
move_regex = re.compile(r'move\s(?P<quantity>\d+)\sfrom\s(?P<source>\d+)\sto\s(?P<dest>\d+)')

class MoveCommand:
    def __init__(self, line: str) -> None:
        g = move_regex.search(line)
        try:
            self.quantity = int(g.group('quantity'))
            self.source = int(g.group('source')) - 1
            self.dest = int(g.group('dest')) - 1
        except:
            print("Could not parse the line")
            quit()

class Storage:
    def __init__(self):
        self.storage = [[], [], [], [], [], [], [], [], []]
    
    def append(self, layer):
        i = 0
        l = [x.group().strip() for x in layer]
        for crate in l:
            if crate.strip() != '':
                self.storage[i].insert(0, crate[1])
            i += 1

    def move(self, move_command: MoveCommand):
        for _ in range(0, move_command.quantity):
            crane = self.storage[move_command.source].pop()
            self.storage[move_command.dest].append(crane)
    
    def solution(self):
        sol = []
        for x in self.storage:
            if len(x) > 0:
                x.reverse()
                sol.append(x[0])
            else:
                continue
        return "".join(sol)


cargo_bay = Storage()
line_number = 0

with open(input_file_path) as f:
    for line in f.readlines():
        line_number += 1
        
        if crate_regex.match(line):
            match = crate_regex.finditer(line)
            cargo_bay.append(match)
            # for m in match:
            #     print(f"Found cid: '{m.group().strip()}'")
        elif move_regex.match(line):
            command = MoveCommand(line)
            cargo_bay.move(command)

        if line_number % 10 == 0:
            print(line_number)

print(cargo_bay.solution())