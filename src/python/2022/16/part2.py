# Quality of life imports
from pathlib import Path
from sys import modules, path


# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

import re
from collections import defaultdict
from functools import lru_cache


helper_location = Path(src, "..", "..", "helpers")
path.insert(1, helper_location.as_posix())
from timer import time_func

#
# So it seems I made a version of this algorithm without knowing it existed for part 1
#   https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
# After doing some research on it I can make mine better by using a different key system
#  for my results
#    Some people are so damned smart...
#


tun_regex = re.compile(r'^Valve (?P<tunnel>[A-Z]{2}) has flow rate=(?P<rate>\d+); tunnels? leads? to valves? (?P<connected>(\D\D,?\s?)+)$')

EXAMPLE_MODE = False
example_answer = 1651
example = """Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"""

starting_node = 'AA'
bitmask = {}
nodes = {'nodes_seen': [starting_node]}

def parse_src(src):
    for line in src:
        w = tun_regex.match(line)
        yield w.group('tunnel'), w.group('rate'), w.group('connected').split(', ')

def generate_max_flow_rates_from_node(current_node, time_left):
    res = []
    for node in nodes['nodes_seen']:
        amount = nodes[node]['rate'] * (time_left - nodes[current_node]['node_distances'][node] -1) # extra -1 for opening the valve
        if amount > 0:
            res.append((node, amount))
    res.sort(key=lambda a: a[1], reverse=True)
    return res

def generate_distance_from_start(node, depth=0):
    if nodes[node]['distance'] > depth:
        nodes[node]['distance'] = depth
    for child in nodes[node]['connected']:
        if child in nodes['nodes_seen'] and depth +1 >= nodes[child]['distance']:
            continue
        if child not in nodes['nodes_seen']:
            nodes['nodes_seen'].append(child)
        generate_distance_from_start(child, depth +1)

# Yes this is basically a duplicate leave me aloooooone
def generate_distance_from_node(node, node_store, nodes_actual, depth=0):
    if node_store[node] > depth:
        node_store[node] = depth
    for child in nodes_actual[node]['connected']:
        if child in node_store['nodes_seen'] and depth +1 >= node_store[child]:
            continue
        node_store['nodes_seen'].append(child)
        generate_distance_from_node(child, node_store, nodes_actual, depth +1)

def generate_inter_node_distances():
    for node in nodes:
        if node == 'nodes_seen': continue
        nodes[node]['node_distances'] = defaultdict(lambda: 1000)
        nodes[node]['node_distances']['nodes_seen'] = [node]
        generate_distance_from_node(node, nodes[node]['node_distances'], nodes)
            
def generate_bitmask():
    i = 0
    for node in nodes:
        if node == 'nodes_seen': continue
        if nodes[node]['rate'] == 0: continue
        bitmask[node] = 1 << i
        i += 1

def filter_options(ans, current_node='AA', time_left=30, state=0, current_total_vented=0):
    flows = generate_max_flow_rates_from_node(current_node, time_left)
    if len(flows) == 0: return
    for target, amount in filter(lambda x: time_left - nodes[current_node]['node_distances'][x[0]] -1 > 0 and not bitmask[x[0]] & state, flows):
        ans[state] = max(ans.get(state, 0), amount + current_total_vented)
        filter_options(
            ans,
            current_node=target, 
            time_left=time_left-nodes[current_node]['node_distances'][target] -1, 
            state= state | bitmask[target], 
            current_total_vented=current_total_vented+amount)

@time_func
def day16part2init(src):
    for tunnel, rate, connected in parse_src(src):
        nodes[tunnel] = {
            'rate': int(rate),
            'connected': connected,
            'distance': 1000,
            'closed': True
        }
        if EXAMPLE_MODE: print(f"Tunnel {tunnel} is connecteded to {connected} and has rate {rate}")
    generate_distance_from_start(starting_node, depth=0)
    generate_inter_node_distances()
    generate_bitmask()

@time_func
def day16part2solve():
    answer = {}
    filter_options(answer, time_left=26)
    sol = max(v1 + v2 for k1, v1 in answer.items() for k2, v2 in answer.items() if not k1 & k2 and v1 + v2 > 2500)
    print(f"Part 2 answer: {sol}")

def part2(src):
    day16part2init(src)
    day16part2solve()
    

if __name__ == "__main__":
    src = []

    if EXAMPLE_MODE:
        src = example.split("\n")
    else:
        with open(input_file_path) as f:
            for line in f.readlines():
                src.append(line.strip('\n'))
    part2(src)
