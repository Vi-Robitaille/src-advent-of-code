# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

w, h = 5, 5


class Board:
    def __init__(self, layout):
        self.board = layout
        self.marked_slots = [0] * (w * h)
        self.has_won = False
        self.index_won_on = -1
        self.winning_number = -1
        self.winning_sum = -1

    def check_won(self):

        if sum(self.marked_slots) <= 4:
            return

        # Check collumns
        for i in range(5):
            if sum([self.marked_slots[i + (x * w)] for x in range(5)]) == 5:
                self.has_won = True
                return

        # Check rows
        for i in range(5):
            if sum(self.marked_slots[i * w:(i * w) + 5]) == 5:
                self.has_won = True
                return

    def cast_number(self, number):
        try:
            if (idx := self.board.index(number)) >= 0:
                self.marked_slots[idx] = 1
                self.check_won()
        except ValueError:
            return

    def calc_answer(self, winning_number):
        if not isinstance(winning_number, int):
            a = int(winning_number)
        else:
            a = winning_number
        unmarked_slots = []
        for i, e in enumerate(self.marked_slots):
            if e != 1:
                unmarked_slots.append(int(self.board[i]))
        self.winning_sum = sum(unmarked_slots) * a

    def process_input(self, input):
        for i, e in enumerate(input):
            if self.has_won:
                return
            self.cast_number(e)
            if self.has_won:
                self.winning_number = int(e)
                self.index_won_on = i
                self.calc_answer(int(e))


input, boards = [], []

with open(input_file_path) as f:
    input = f.readline().strip().split(',')

    for i in range(100):
        # Trash empty line
        f.readline()

        b = []
        for i in range(h):
            b += f.readline().rstrip().split()
        boards.append(Board(b))

for b in boards:
    b.process_input(input)


s_boards = sorted(boards, key=lambda x: x.index_won_on)

print("I accidentallied part 2 while i was doing part 1...")

# Print last solved board
print("The first board is:", s_boards[0].winning_sum)

# Print first solved board
print("The last board is:", s_boards[-1].winning_sum)
