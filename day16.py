"""Day 16: Proboscidea Volcanium
For part 1, I start from the understanding that the search space boils down to
different permutations of non-zero valves, since for each sequence of valves,
we can use "shortest distance between valves" to compute the total release if
we travel to each valve according to that sequence. The search space can be
further reduced if we consider the time constraint.

For part 2, the trick is to understand that the valves opened by me and the 
valves opened by the elephant are disjoint sets. For each split of valves
between me and the elephant, each of us can independently find the optimal
sequence of valves to go through, then the total release is the sum of 
cumulative releases between the two of us. We can then simply search through
all possible splits and find the optimal solution there.
"""
import re
from typing import Tuple, Dict, List
from itertools import combinations

def parse_inputs(input_path: str):
    with open(input_path) as f:
        inputs = f.read()
    valves = []
    rates = {}  # maps valve to rate
    adj = {}  # maps (valve, valve) to 1 or inf
    for line in inputs.splitlines():
        valve, rate, neighbors = parse_line(line)
        valves.append(valve)
        rates[valve] = rate
        adj[(valve, valve)] = 0
        for neighbor in neighbors:
            adj[(valve, neighbor)] = adj[(neighbor, valve)] = 1.
    
    return valves, rates, adj

def parse_line(line: str) -> Tuple[str, int, List[str]]:
    result = re.search(
        r"Valve (.+) has flow rate=(.+); tunnel[s]* lead[s]* to valve[s]* (.*)",
        line
    )
    result = result.groups()

    return result[0], int(result[1]), result[2].split(", ")

def floyd_warshall(valves: List[str], adj: Dict[Tuple[str, str], float]):
    """Find the shortest path between all pairs of valves"""
    dists = {}
    for i in valves:
        for j in valves:
            dists[(i, j)] = adj.get((i, j), float("Inf"))
    
    for i in valves:
        for j in valves:
            for k in valves:
                dists[(j, k)] = dists[(k, j)] = min(
                    dists[(j, k)], dists[(j, i)] + dists[(i, k)]
                )
    return dists

def get_max_release(cur_valve, t_remain, footprints, dists, rates):
    """Return the maximal achievable cumulative release given that the search
    begins at "cur_valve", there is t_remain minutes remaining. "footprints"
    record the set of opened and empty valves that don't need to be searched
    again
    """
    max_release = 0

    if len(footprints) == len(rates):
        return 0
    if t_remain <= 0:
        return 0
    for next_valve in [v for v in rates if v not in footprints]:
        t_travel = dists[(cur_valve, next_valve)]
        if t_remain >= t_travel + 1:
            next_max_release = get_max_release(
                next_valve, t_remain - t_travel - 1,
                footprints | {next_valve: True}, dists, rates
            )
            cum_rel = rates[next_valve] * (t_remain - t_travel - 1)
            if cum_rel + next_max_release > max_release:
                max_release = cum_rel + next_max_release

    return max_release

if __name__ == "__main__":
    valves, rates, adj = parse_inputs("inputs/16.txt")
    non_empty_valves = [v for v in valves if rates[v] > 0]
    dists = floyd_warshall(valves, adj)
   
    # part 1
    import time
    start = time.monotonic()
    print(
        get_max_release(
            "AA", 30, {v: True for v in valves if rates[v] == 0}, dists, rates
        )
    )
    stop1 = time.monotonic()
    print(f"{stop1 - start:.4f} seconds")

    # part 2
    max_release = 0
    for n in range(1, len(non_empty_valves) // 2):
        # Combination has manageable time complexity
        for my_valves in combinations(non_empty_valves, n):
            elephant_valves = [v for v in non_empty_valves if v not in my_valves]
            my_max_rel = get_max_release("AA", 26, 
                {v: True for v in valves if rates[v] == 0 or v in elephant_valves},
                dists, rates)
            el_max_rel = get_max_release("AA", 26, 
                {v: True for v in valves if rates[v] == 0 or v in my_valves},
                dists, rates)
            if my_max_rel + el_max_rel > max_release:
                max_release = my_max_rel + el_max_rel
    print(max_release)
    stop2 = time.monotonic()
    print(f"{stop2 - stop1:.4f} seconds")
