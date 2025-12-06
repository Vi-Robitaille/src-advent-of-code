# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

# I hate the fish thing...
# No effort to make it pretty

winning_score = 21

player_one_pos = -1
player_one_score = 0
player_two_pos = -1
player_two_score = 0

player_one_wins = 0
player_two_wins = 0


def get_position_for_roll(current_position, roll):
    if (score := (current_position + roll) % 10) == 0:
        return 10
    else:
        return score


# ITS THE FUCKING FISH PROBLEM AGAIN
with open(input_file_path) as f:
    line = f.readline().rstrip()
    player_one_pos = int(line.split(':')[-1])
    line = f.readline().rstrip()
    player_two_pos = int(line.split(':')[-1])

# Occurances of each roll
die = {
    3: 1,
    4: 3,
    5: 6,
    6: 7,
    7: 6,
    8: 3,
    9: 1
}

universes = {
    ((player_one_pos, player_one_score), (player_two_pos, player_two_score)): 1
}


def do_turn(universes, turn):
    # Globals are bad, bite me
    global player_one_wins
    global player_two_wins

    new_universes = {}
    for key in universes:
        start_pos = key[turn][0]
        start_score = key[turn][1]
        for i in die:
            num_new_universes = 0
            pos = get_position_for_roll(start_pos, i)
            score = start_score + pos
            num_new_universes = universes[key] * die[i]
            if score >= winning_score:
                if turn == 0:
                    player_one_wins += num_new_universes
                else:
                    player_two_wins += num_new_universes
                continue
            if turn == 0:
                new_universe = ((pos, score), key[1])
            else:
                new_universe = (key[0], (pos, score))
            if new_universe in new_universes:
                new_universes[new_universe] += num_new_universes
            else:
                new_universes[new_universe] = num_new_universes
    return new_universes


end = False
turn = 0  # 0 for player 1 , 1 for player 2
while not end:
    universes = do_turn(universes, turn)
    if len(universes) == 0:
        break
    if turn == 0:
        turn = 1
    else:
        turn = 0

more_wins = max([player_one_wins, player_two_wins])
print(f"The most wins is: {more_wins}")
