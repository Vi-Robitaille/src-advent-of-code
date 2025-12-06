# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

from itertools import cycle


DICE_SIZE = 100
die = cycle(range(1, DICE_SIZE + 1))
dice_rolls = 0
winning_score = 1000

player_one_pos = -1
player_one_score = 0
player_two_pos = -1
player_two_score = 0


def get_rolls():
    global dice_rolls
    dice_rolls += 3
    return next(die) + next(die) + next(die)

def get_position_for_roll(current_position, roll):
    if (score := (current_position + roll) % 10) == 0:
        return 10
    else:
        return score

with open(input_file_path) as f:
    line = f.readline().rstrip()
    player_one_pos = int(line.split(':')[-1])
    line = f.readline().rstrip()
    player_two_pos = int(line.split(':')[-1])


while player_one_score < winning_score or player_two_score < winning_score:
    rolls = get_rolls()
    player_one_pos = get_position_for_roll(player_one_pos, rolls)
    player_one_score += player_one_pos
    if player_one_score >= winning_score:
        break
    
    rolls = get_rolls()
    player_two_pos = get_position_for_roll(player_two_pos, rolls)
    player_two_score += player_two_pos
    if player_two_score >= winning_score:
        break

result = min([player_one_score, player_two_score]) * dice_rolls
print(f"The result is: {result}")