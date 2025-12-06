
# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "ex-input.txt")

from helpers import Node
import re
import pyparsing

def create_equation(string):
    matchchars = pyparsing.Word(pyparsing.alphanums) | ','
    patterns   = pyparsing.nestedExpr( '[', ']', content=matchchars)
    parsed = patterns.parseString(string).asList()
    return parsed

# with open(input_file_path) as f:
#     lines = [x.rstrip() for x in f.readlines()]
a = "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]"
b = "[[1,2],3]"
source = create_equation(b)
master_node = Node(source)
print()
