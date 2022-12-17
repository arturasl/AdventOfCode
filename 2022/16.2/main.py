import sys
import re
from heapq import *
from collections import namedtuple
from collections import defaultdict

best = []

def set_val(state, val):
    global best
    best[state.time_left][state.current][state.opened] = val

def get_val(state):
    return best[state.time_left][state.current][state.opened]

Valve = namedtuple('Valve', ['name', 'rate', 'other'])
Edge = namedtuple('Edge', ['to', 'cost'])
class State(namedtuple('State', ['current', 'opened', 'time_left'])):
    def __lt__(self, other):
        global best
        return (get_val(self), self.time_left) > (get_val(other), other.time_left)

def read():
    valve_map = {}
    r = re.compile(r'^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ((?:\w+, )*\w+)$')
    for line in [line.strip() for line in sys.stdin if line]:
        m = re.match(r, line)
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

    def add_new_state(new_flow, state, new_edge, new_opened):
        new_state = State(
                current=new_edge.to,
                opened=new_opened,
                time_left=state.time_left - new_edge.cost)
        if new_state.time_left <= 0:
            return
        if get_val(new_state) < new_flow:
            set_val(new_state, new_flow)
            heappush(queue, new_state)

    max_time_left = 26
    state = State(current=valve_to_idx['AA'], opened=0, time_left=max_time_left)
    queue = [state]
    best = [[[-1 for _ in range(1 << idx)] for _ in range(len(valves))] for _ in range(max_time_left + 1)]
    set_val(state, 0)

    while queue:
        state = heappop(queue)
        cur_flow = get_val(state)

        if valves[state.current].rate != 0 and not(state.opened & (1 << ovalve_to_idx[state.current])):
            add_new_state(
                    new_flow=cur_flow + (state.time_left - 1) * valves[state.current].rate,
                    state=state,
                    new_edge=Edge(to=state.current, cost=1),
                    new_opened=state.opened | (1 << ovalve_to_idx[state.current]))

        for edge in valves[state.current].other:
            add_new_state(
                    new_flow=cur_flow,
                    state=state,
                    new_edge=edge,
                    new_opened=state.opened)

    best_p = [-1 for _ in range(1 << idx)]
    for t in range(max_time_left + 1):
        for e in range(len(valves)):
            for o in range(1 << idx):
                best_p[o] = max(best_p[o], best[t][e][o])

    global_result = 0
    for i in range(1 << idx):
        mask = ~i & ((1 << idx) - 1)
        j = mask
        while j != 0:
            global_result = max(global_result, best_p[i] + best_p[j])
            j = (j - 1) & mask

    print(f'Best: {global_result}')

if __name__ == "__main__":
    main()
