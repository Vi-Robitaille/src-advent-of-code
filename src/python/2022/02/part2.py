# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

from enum import IntEnum

example = """A Y
B X
C Z"""

# mental note to make things clear
LHS = {
    "A": 1, # Rock
    "B": 2, # Paper
    "C": 3, # Scissors
}

RHS = {
    "X": 1, # Lose
    "Y": 2, # Draw
    "Z": 3, # Win
}

class GameOutcome(IntEnum):
    LOSS = 0
    DRAW = 3
    WIN = 6

game_outcomes = {
    ("A", GameOutcome.LOSS): 3 + GameOutcome.LOSS,
    ("A", GameOutcome.DRAW): 1 + GameOutcome.DRAW,
    ("A", GameOutcome.WIN):  2 + GameOutcome.WIN,

    ("B", GameOutcome.LOSS): 1 + GameOutcome.LOSS,
    ("B", GameOutcome.DRAW): 2 + GameOutcome.DRAW,
    ("B", GameOutcome.WIN):  3 + GameOutcome.WIN,

    ("C", GameOutcome.LOSS): 2 + GameOutcome.LOSS,
    ("C", GameOutcome.DRAW): 3 + GameOutcome.DRAW,
    ("C", GameOutcome.WIN):  1 + GameOutcome.WIN,
}

game_counts = {
}

total = 0

def map_rhs_outcome(rhs):
    match rhs:
        case "X":
            return GameOutcome.LOSS
        case "Y":
            return GameOutcome.DRAW
        case "Z":
            return GameOutcome.WIN
        case _:
            raise ValueError("Invalid input.")

with open(input_file_path) as f:
    for line in f.readlines():
        temp = line.strip()
        if temp in game_counts.keys():
            game_counts[temp] += 1
        else:
            game_counts[temp] = 1

for key, value in game_counts.items():
    lhs = key[0]
    rhs = key[2]
    total += game_outcomes[(lhs, map_rhs_outcome(rhs))] * value

print(f"The total score is {total}")

