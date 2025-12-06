import math


class Node:
    def __init__(self, value=None, parent=None) -> None:

        self.parent = parent
        self._depth = 0 if parent is None else parent.depth + 1

        while len(value) == 1 and isinstance(value, list):
            value = value[0]
            self._depth += 1

        if isinstance(value[0], str):
            self.left_value = int(value[0])
        elif isinstance(value[0], list):
            self.left_value = Node(value[0], parent=self)

        if isinstance(value[2], str):
            self.right_value = int(value[2])
        elif isinstance(value[2], list):
            self.right_value = Node(value[2], parent=self)
    
    @property
    def depth(self):
        if self.parent == None:
            self._depth = 0
            return 0
        else:
            self._depth = self.parent.depth + 1
            return self._depth
        
def add(a: Node, b: Node) -> Node:
    tmp = Node()
    tmp.left_value = a
    tmp.right_value = b
    return

def split(value_to_split: int, parent_node: Node) -> Node:
    tmp = Node(parent=parent_node)
    tmp.left_value = int(value_to_split / 2)
    tmp.right_value = math.ceil(value_to_split / 2)
    return tmp

def explode(exploder: Node):
    eval_node = exploder.parent
    