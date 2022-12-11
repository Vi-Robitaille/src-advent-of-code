# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

from math import floor, prod

EXAMPLE_MODE = False
example_answer = 10605
example = \
"""Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"""

class Operation:
    def __init__(self, operation: str):
        self.op = operation.split(' ')[1]
        self.value = operation.split(' ')[2]

    def calc(self, current_value):
        if self.value == "old":
            value = current_value
        else:
            value = int(self.value)
        match self.op:
            case "*": return current_value * value
            case "+": return current_value + value

class Monkey:
    def __init__(self, inp: dict):
        self.held_items = [int(x) for x in str(inp['held_items']).split(',')]
        self.operation = Operation(inp['operation'])
        self.test = inp['test']
        self.true_action = inp['true_action']
        self.true_target = None
        self.false_action = inp['false_action']
        self.false_target = None
        self.inspected_items = 0


    def setup_target_monkeys(self, all_monkeys):
        self.true_target = all_monkeys[self.true_action]
        self.false_target = all_monkeys[self.false_action]

    def inspect_items(self):
        self.inspected_items += len(self.held_items)
        self.held_items = [floor(self.operation.calc(elem) / 3) for elem in self.held_items]
    
    def throw_items(self):
        for item in self.held_items:
            if item % self.test == 0:
                self.true_target.held_items.append(item)
            else:
                self.false_target.held_items.append(item)
        self.held_items = []

    def do_turn(self):
        if self.true_target == None or self.false_target == None:
            raise BaseException("Not initialized")
        if len(self.held_items) == 0:
            return
        self.inspect_items()
        self.throw_items()


def part1(input):
    monkey_list = [Monkey(p) for _, p in input.items()]
    [m.setup_target_monkeys(monkey_list) for m in monkey_list]
    for _ in range(0, 20):
        [m.do_turn() for m in monkey_list]
    sorted_monkeys = sorted(monkey_list, key=lambda x: x.inspected_items, reverse=True)[:2]
    print(f"Found {prod([x.inspected_items for x in sorted_monkeys])} MONKEY BUSINESS!")

def read_input():
    from yaml import load
    try:
        from yaml import CLoader as Loader
    except ImportError:
        from yaml import Loader
    if EXAMPLE_MODE:
        input = example
    else:
        with open(input_file_path) as f:
            input = f.read()
    corrected = input\
            .replace("  If true", "true_action")\
            .replace("  If false", "false_action")\
            .replace("throw to monkey ", "")\
            .replace("divisible by ", "")\
            .replace("new = ", "")\
            .replace("Starting items", "held_items")\
            .replace("Operation", "operation")\
            .replace("Test", "test")
    input = load(corrected, Loader=Loader)
    return input

if __name__ == "__main__":
    input = read_input()
    part1(input)
