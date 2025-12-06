# Quality of life imports
from pathlib import Path
from sys import modules

# Quality of life, define the input file location
src = Path(modules['__main__'].__file__).resolve().parent
input_file_path = Path(src, "input.txt")

import re
from collections import defaultdict
from functools import lru_cache

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


def parse_src(src):
    for line in src:
        w = tun_regex.match(line)
        yield w.group('tunnel'), w.group('rate'), w.group('connected').split(', ')

def generate_max_flow_rates_from_node(current_node, time_left, nodes):
    res = []
    for node in nodes['nodes_seen']:
        amount = nodes[node]['rate'] * (time_left - nodes[current_node]['node_distances'][node] -1) # extra -1 for opening the valve
        if amount > 0:
            res.append((node, amount))
    res.sort(key=lambda a: a[1], reverse=True)
    return res

def generate_distance_from_start(node, nodes, depth=0):
    if nodes[node]['distance'] > depth:
        nodes[node]['distance'] = depth
    for child in nodes[node]['connected']:
        if child in nodes['nodes_seen'] and depth +1 >= nodes[child]['distance']:
            continue
        if child not in nodes['nodes_seen']:
            nodes['nodes_seen'].append(child)
        generate_distance_from_start(child, nodes, depth +1)

# Yes this is basically a duplicate leave me aloooooone
def generate_distance_from_node(node, node_store, nodes_actual, depth=0):
    if node_store[node] > depth:
        node_store[node] = depth
    for child in nodes_actual[node]['connected']:
        if child in node_store['nodes_seen'] and depth +1 >= node_store[child]:
            continue
        node_store['nodes_seen'].append(child)
        generate_distance_from_node(child, node_store, nodes_actual, depth +1)

def generate_inter_node_distances(nodes):
    for node in nodes:
        if node == 'nodes_seen': continue
        nodes[node]['node_distances'] = defaultdict(lambda: 1000)
        nodes[node]['node_distances']['nodes_seen'] = [node]
        generate_distance_from_node(node, nodes[node]['node_distances'], nodes)
            
def filt(x):
    if x is None:
        return 0
    else:
        x[1]

def filter_options(nodes, ans, current_node='AA', time_left=30, opened_nodes=[], current_total_vented=0):
    opened_nodes.append(current_node)
    flows = generate_max_flow_rates_from_node(current_node, time_left, nodes)
    if len(flows) == 0:
        return
    for target, amount in filter(lambda x: time_left - nodes[current_node]['node_distances'][x[0]] -1 > 0 and x[0] not in opened_nodes, flows):
        ans[",".join([*opened_nodes, target])] = amount+current_total_vented
        on = opened_nodes.copy()
        filter_options(
            nodes, 
            ans,
            current_node=target, 
            time_left=time_left-nodes[current_node]['node_distances'][target] -1, 
            opened_nodes=on, 
            current_total_vented=current_total_vented+amount)

def part1(src):
    starting_node = 'AA'
    nodes = {'nodes_seen': [starting_node]}
    for tunnel, rate, connected in parse_src(src):
        nodes[tunnel] = {
            'rate': int(rate),
            'connected': connected,
            'distance': 1000,
            'closed': True
        }
        print(f"Tunnel {tunnel} is connecteded to {connected} and has rate {rate}")
    generate_distance_from_start(starting_node, nodes, depth=0)
    generate_inter_node_distances(nodes)
    answer = {}
    filter_options(nodes, answer)
    print(f"OOF THIS IS ROUGH {max(answer.values())}")

if __name__ == "__main__":
    src = []

    if EXAMPLE_MODE:
        src = example.split("\n")
    else:
        with open(input_file_path) as f:
            for line in f.readlines():
                src.append(line.strip('\n'))
    part1(src)
