# 
# 
# I scavanged this from the first python thing i wrote ages ago
# I also didnt take the time to make this pretty so you're gonan cringe
# 
# 

import heapq
import pygame
import sys
from math import floor, sqrt
import random

WALL_WIDTH = 2 * 2
PATH_WIDTH = 4 * 2
DRAW_OFFSET = WALL_WIDTH + PATH_WIDTH

# Yes i know i should infer this from the input, but i didnt
MAZE_WIDTH = 0
MAZE_HEIGHT = 0
DO_DRAW_CALLS = True


BLACK = (0, 0, 0)
RED = (255, 0, 0)
GREEN = (0, 255, 0)
BLUE = (0, 0, 255)
GREY = (185, 185, 185)
WHITE = (255, 255, 255)

class World:
    def __init__(self, source: str):
        self.cells = [Cell(idx, element) for idx, element in enumerate(source)]
        self.start = 0 # randint(0, MAZE_WIDTH * MAZE_HEIGHT)
        self.end = MAZE_WIDTH * MAZE_HEIGHT - 1 # randint(0, MAZE_WIDTH * MAZE_HEIGHT)
        self.cells[self.start].is_start = True
        self.cells[self.end].is_end = True
        for cell in self.cells:
            cell.populate_neighbors(self.cells)
            cell.draw_position = (cell.position[0] * DRAW_OFFSET + sqrt(PATH_WIDTH),
                                  cell.position[1] * DRAW_OFFSET + sqrt(PATH_WIDTH))

    def update(self, stack):
        this_cell = stack[-1]
        this_cell.visited = True
        possible_next_cells = [i for i in this_cell.neighbors if not i.visited and i is not None]
        if len(possible_next_cells) == 0:
            stack.pop()
            return
        next_cell = random.choice(possible_next_cells)
        stack.append(next_cell)


class Cell:
    def __init__(self, idx, element):
        # Base Cell class other cells will morph if needed by a search type
        self.id = idx
        self.neighbors = []
        self.is_start = False
        self.is_end = False
        self.visited = False
        self.parent = None
        self.number_of_parents = 0
        # [0] = x, [1] = y
        self.position = (idx % MAZE_WIDTH, floor(idx / MAZE_WIDTH))
        self.draw_position = (-1, -1)
        # East and South
        # We do not need to care about all directions on each cell as wither it's neighbors will handle that
        # or it does not have neighbors in that direction
        self.path = [False, False]

        # The cost of this cell
        self.g = int(element)

        # The heuristic value
        self.h = 999999999999
        
        # The total cost of this cell (all cells in its path + this)
        # Default should be real big so we overwrite it with the first visitor
        self.f = 999999999999

    def populate_neighbors(self, cells):
        # Check if the cell has neighbors in each cardinal direction
        # then assigns them as its neighbors or None keeping indexing as an option for referencing
        # direction, ex: north = 0 cell.neighbors[north]
        # (This is not implemented yet. However, it will add an additional layer of clarity to the code)
        north = self.id - MAZE_WIDTH if self.position[1] > 0 else None
        east = self.id + 1 if self.position[0] < MAZE_WIDTH - 1 else None
        south = self.id + MAZE_WIDTH if self.position[1] < MAZE_HEIGHT - 1 else None
        west = self.id - 1 if self.position[0] > 0 else None
        self.neighbors = [cells[i] for i in [north, east, south, west] if i is not None]

    def __lt__(self, other):
        # We need to create a less than comparison as the heapq will use it to order
        # each cell. Without it the heapq has no idea how to compare the cells
        if isinstance(other, Cell):
            # return (self.f * self.h) < (other.f * other.h)
            return self.f < other.f


class Search:
    def __init__(self, world, screen):
        self.world = world
        self.cells = self.world.cells
        self.screen = screen
        self.opened = []
        self.closed = set()
        self.solved = False
        self.path = []
        self.end = self.cells[self.world.end]
        self.start = self.cells[self.world.start]

    def get_reachable(self, cell):
        """
        Since everything is reachable 
        :param cell: The current cell being evaluated
        :return: A list of cardinal directions in clockwise order starting at north,
                    if the cell is not reachable it returns (False, None) for that cell
        """
        north = south = east = west = (False, None)
        for neighbor in cell.neighbors:
            if neighbor.id == cell.id - MAZE_WIDTH:
                north = (True, neighbor)
            elif neighbor.id == cell.id + MAZE_WIDTH:
                south = (True, neighbor)
            elif neighbor.id == cell.id - 1:
                west = (True, neighbor)
            elif neighbor.id == cell.id + 1:
                east = (True, neighbor)
        return [north, east, south, west]

    def get_path(self):
        # Iterates through the gathered cells by jumping to its parent
        # constructing a list of cells on the path between the start and end
        cell = self.end
        path = [cell]
        while cell.parent is not self.start:
            cell = cell.parent
            path.append(cell)
        path.append(self.start)
        self.path = path

    def draw_cell(self, cell, color=WHITE, update=False):
        if not DO_DRAW_CALLS or not self.solved:
            return
        color = (255 / (cell.g + 1), 127 / (cell.g + 1), 0)
        pygame.draw.rect(self.screen, color,
                         (cell.draw_position[0],
                          cell.draw_position[1],
                          PATH_WIDTH, PATH_WIDTH))
        if update:
            pygame.display.update()

    # def update_cell(self, cell, neighbor):
    #     if neighbor.f > cell.f + neighbor.g:
    #     # if neighbor.parent is None:
    #         neighbor.parent = cell

    # TODO: Check later
    def draw(self):
        if not DO_DRAW_CALLS or not self.solved:
            return
        for cell in self.cells:
            if cell in self.closed:
                color = GREY
                if cell is self.start:
                    color = BLUE
                if cell is self.end:
                    color = RED
                self.draw_cell(cell, color)
        if self.path:
            divisions = 255 / len(self.path)
            color = RED
            for cell in self.path:
                if cell.parent is None:
                    continue
                color = (color[0] - divisions if color[0] - divisions > 0 else 0, 0,
                         color[2] + divisions if color[2] + divisions < 255 else 255)
                pygame.draw.line(self.screen, color,
                                 (cell.draw_position[0] + PATH_WIDTH / 2,
                                  cell.draw_position[1] + PATH_WIDTH / 2),
                                 (cell.parent.draw_position[0] + PATH_WIDTH / 2,
                                  cell.parent.draw_position[1] + PATH_WIDTH / 2),
                                 floor(WALL_WIDTH / 2))


class StarSearch(Search):
    def __init__(self, world, screen):
        super().__init__(world, screen)
        # Init the search, we need to rebuild the set of cells as they need more info stored in them
        heapq.heapify(self.opened)

        heapq.heappush(self.opened, self.start)


    def get_heuristic(self, cell):
        """
        Calculate the heuristic value H for a cell: dist between this cell and the ending cell x 10
        :param cell: the cell of which to calculate the heuristic
        :returns heuristic value:
        """

        a = pow(self.end.position[1] - cell.position[1], 2)
        b = pow(cell.position[0] - self.end.position[0], 2)
        cell.h = sqrt(a + b)


    def remove_element(self, cell):
        idx = self.opened.index(cell)
        self.opened[idx] = self.opened[-1]
        self.opened.pop()
        heapq.heapify(self.opened)

    def update_cell(self, adj, cell):
        """
        Updates a cells' heuristic, cost and parent
        :param adj: The cell adjacent to the current cell to be updated
        :param cell: The current cell being evaluated
        :return: None
        """

        if adj.h == 999999999999:
            self.get_heuristic(adj)

        adj.parent = cell
        adj.f = cell.f + adj.g
        adj.number_of_parents = 1 + cell.number_of_parents


    def update(self):
        """
        The big worky worky bit
        Iterates over each cell, working with the most efficient path via the heapq sorting the
        lowest costing (f) cell to the top to be evaluated next.
        """
        if not self.path:
            cell = heapq.heappop(self.opened)
            self.closed.add(cell)
            if cell is self.end:
                self.get_path()
                self.solved = True
            self.draw_cell(cell, BLUE)
            for neighbor in cell.neighbors:
                if neighbor in self.closed:
                    continue

                # If the cell we're looking at has a higher cost already than the one we can give it
                # override that cost
                elif neighbor in self.opened and neighbor.f > cell.f + neighbor.g:
                    self.remove_element(neighbor)
                    self.update_cell(neighbor, cell)
                    heapq.heappush(self.opened, neighbor)
                elif neighbor not in self.opened:
                    # Something isnt right here....?
                    self.update_cell(neighbor, cell)
                    heapq.heappush(self.opened, neighbor)
                self.draw_cell(neighbor, GREEN)
        else:
            return

class PyGameObj():
    def __init__(self, input_str, do_draw_calls=True):
        # Set up the core object used to draw to the screen and hold the maze data


        # I'm basically on a roll for crimes at this point
        global MAZE_HEIGHT
        global MAZE_WIDTH
        global DO_DRAW_CALLS

        size = sqrt(len(input_str))
        MAZE_WIDTH = int(size)
        MAZE_HEIGHT = int(size)

        if not do_draw_calls or size > 200:
            DO_DRAW_CALLS = do_draw_calls

        if do_draw_calls or size < 200:
            pygame.init()
            pygame.display.set_caption("Mathieu Robitaille's Maze Generation")
            x_size = MAZE_WIDTH * (WALL_WIDTH + PATH_WIDTH)
            y_size = MAZE_HEIGHT * (WALL_WIDTH + PATH_WIDTH)
            self.screen = pygame.display.set_mode((y_size, x_size))


        self.w = World(input_str)
        self.w.cells[0].g = 0
        self.w.cells[0].f = 0

        self.search = StarSearch(self.w, self.screen)


    def run(self):
        # This holds the main while loop for the maze generation and the control logic for which
        # search algorithm will solve the maze
        while not self.search.solved:
            if DO_DRAW_CALLS:
                self.event_loop()

            # Do stuff
            self.update()

            # Draw the stuff you did
            if DO_DRAW_CALLS:
                self.draw()
                self.search.draw()
        
            if DO_DRAW_CALLS:
                # Place update call here as update may have some specific
                # draw calls to make we do not want over write
                pygame.display.update()

    def event_loop(self):
        # Check if the user is trying to exit the pygame instance
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                sys.exit()

    def draw(self):
        self.screen.fill(pygame.Color("black"))
        

    def update(self):
        self.search.update()


