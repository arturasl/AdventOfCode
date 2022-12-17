import sys
import re
import random
from heapq import *
from collections import namedtuple
from collections import defaultdict
from functools import total_ordering

# best = []
best = defaultdict(lambda: -1)

def set_val(state, val):
    global best
    # best[state.time_left[0]][state.time_left[1]][state.current[0]][state.current[1]][state.opened] = val
    best[state] = val

def get_val(state):
    # return best[state.time_left[0]][state.time_left[1]][state.current[0]][state.current[1]][state.opened]
    return best[state]

Valve = namedtuple('Valve', ['name', 'rate', 'other'])
Edge = namedtuple('Edge', ['to', 'cost'])
@total_ordering
class State(namedtuple('State', ['current', 'opened', 'time_left'])):
    def __lt__(self, other):
        global best
        return (get_val(self), sum(self.time_left)) > (get_val(other), sum(other.time_left))

def read():
    valve_map = {}
    for line in [line.strip() for line in sys.stdin if line]:
        m = re.match(r'^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ((?:\w+, )*\w+)$', line)
        assert m
        m = m.groups()
        valve_map[m[0]] = Valve(
            name=m[0],
            rate=int(m[1]),
            other=[Edge(to=x, cost=1) for x in m[2].replace(' ', '').split(',')])

    while True:
        to_remove = None
        for valve in valve_map.values():
            if valve.rate == 0 and valve.name != 'AA':
                to_remove = valve
                break
        if to_remove is None:
            break

        del valve_map[to_remove.name]

        for name in valve_map.keys():
            if not any(edge.to == to_remove.name for edge in valve_map[name].other):
                continue
            cost = min(edge.cost for edge in valve_map[name].other if edge.to == to_remove.name)
            other = [edge for edge in valve_map[name].other if edge.to != to_remove.name] + [Edge(to=edge.to, cost=edge.cost + cost) for edge in to_remove.other if edge.to != name]
            cheapest = defaultdict(lambda: 1_000_000)
            for edge in other:
                cheapest[edge.to] = min(cheapest[edge.to], edge.cost)
            valve_map[name] = Valve(
                name=name,
                rate=valve_map[name].rate,
                other=[Edge(to=to, cost=cost) for to, cost in cheapest.items()]
            )

    for valve in valve_map.values():
        print(valve)

    return list(valve_map.values())

def main():
    global best
    valve_list = read()

    valve_to_idx = {}
    for i, valve in enumerate(valve_list):
        valve_to_idx[valve.name] = i

    valves = []
    for valve in valve_list:
        valves.append(
                Valve(
                    name=valve.name,
                    rate=valve.rate,
                    other=[Edge(to=valve_to_idx[edge.to], cost=edge.cost) for edge in valve.other]))

    idx = 0
    ovalve_to_idx = []
    for valve in valve_list:
        ovalve_to_idx.append(idx)
        if valve.rate != 0:
            idx += 1

    num_agents = 1
    max_time_left = 26

    print((1 << idx) * (len(valves) ** (num_agents)) * ((max_time_left + 1) ** num_agents))
    # best = [[[[[-1 for _ in range(1 << idx)] for _ in range(len(valves))] for _ in range(len(valves))] for _ in range(max_time_left + 1)] for _ in range(max_time_left + 1)]


    def add_new_state(player, new_flow, state, new_edge, new_opened):
        new_state = State(
                current=tuple(o if i != player else new_edge.to for i, o in enumerate(state.current)),
                opened=new_opened,
                time_left=tuple(t if i != player else t - new_edge.cost for i, t in enumerate(state.time_left)))
        if new_state.time_left[player] <= 0:
            return
        if len(new_state.current) > 1 and new_state.current[0] > new_state.current[1]:
            new_state = State(
                    current=tuple(new_state.current[::-1]),
                    opened=new_state.opened,
                    time_left=tuple(new_state.time_left[::-1]))
        if get_val(new_state) < new_flow:
            set_val(new_state, new_flow)
            heappush(queue, new_state)

    global_result = 0
    partitions = list(range(1 << idx))
    random.shuffle(partitions)
    for i in partitions:
        local_result = 0
        for partition in [i, ~i & ((1 << idx) - 1)]:
            state = State(current=tuple(valve_to_idx['AA'] for _ in range(num_agents)), opened=partition, time_left=tuple(max_time_left for _ in range(num_agents)))
            queue = [state]
            best = defaultdict(lambda: -1)
            set_val(state, 0)

            while queue:
                state = heappop(queue)
                for player in range(num_agents):
                    if state.time_left[player] <= 1:
                        continue
                    cur_flow = get_val(state)

                    if valves[state.current[player]].rate != 0 and not(state.opened & (1 << ovalve_to_idx[state.current[player]])):
                        add_new_state(
                                player=player,
                                new_flow=cur_flow + (state.time_left[player] - 1) * valves[state.current[player]].rate,
                                state=state,
                                new_edge=Edge(to=state.current[player], cost=1),
                                new_opened=state.opened | (1 << ovalve_to_idx[state.current[player]]))

                    for edge in valves[state.current[player]].other:
                        add_new_state(
                                player=player,
                                new_flow=cur_flow,
                                state=state,
                                new_edge=edge,
                                new_opened=state.opened)
            local_result += max(best.values())

        global_result = max(local_result, global_result)
        print(f'{i:b}: local result {local_result}, global result {global_result}')

    print(f'Best: {global_result}')

if __name__ == "__main__":
    main()
