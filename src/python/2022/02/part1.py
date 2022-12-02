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

class GameOutcome(IntEnum):
    LOSS = 0
    DRAW = 3
    WIN = 6

LHS = {
    "A": 1, # Rock
    "B": 2, # Paper
    "C": 3, # Scissors
}

RHS = {
    "X": 1, # Rock
    "Y": 2, # Paper
    "Z": 3, # Scissors
}

game_counts = {
}

total = 0

def simulate_game(lhs: str, rhs: str):
    match (lhs, rhs):
        case ("A", "Y"):
            return GameOutcome.WIN
        case ("A", "Z"):
            return GameOutcome.LOSS

        case ("B", "Z"):
            return GameOutcome.WIN
        case ("B", "X"):
            return GameOutcome.LOSS

        case ("C", "X"):
            return GameOutcome.WIN
        case ("C", "Y"):
            return GameOutcome.LOSS

        case _:
            return GameOutcome.DRAW
        # case ("A", "X"):
        #     return GameOutcome.DRAW
        # case ("B", "Y"):
        #     return GameOutcome.DRAW
        # case ("C", "Z"):
        #     return GameOutcome.DRAW
        

def calculate_score(game_outcome: GameOutcome, rhs):
    return game_outcome + RHS[rhs]

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
    outcome = simulate_game(lhs, rhs)
    score = calculate_score(outcome, rhs)
    total += score * value

print(f"The total score is {total}")

