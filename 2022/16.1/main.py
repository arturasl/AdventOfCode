import sys
import re
import heapq
from collections import namedtuple
from collections import defaultdict

Valve = namedtuple('Valve', ['name', 'rate', 'other'])
State = namedtuple('State', ['current', 'opened', 'time_left'])

def main():
    valve_list = []
    valve_to_idx = {}
    for line in [line.strip() for line in sys.stdin if line]:
        m = re.match(r'^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ((?:\w+, )*\w+)$', line)
        assert m
        m = m.groups()
        valve_to_idx[m[0]] = len(valve_list)
        valve_list.append(Valve(m[0], int(m[1]), m[2].replace(' ', '').split(',')))

    valves = []
    for valve in valve_list:
        valves.append(Valve(valve.name, valve.rate, [valve_to_idx[name] for name in valve.other]))

    idx = 0
    ovalve_to_idx = []
    for valve in valve_list:
        ovalve_to_idx.append(idx)
        if valve.rate != 0:
            idx += 1

    best = [[[-1 for _ in range(1 << idx)] for _ in range(len(valves))] for _ in range(30 + 1)]
    state = State(current=valve_to_idx['AA'], opened=0, time_left=30)
    queue = [state]
    best[state.time_left][state.current][state.opened] = 0
    its = 0
    while queue:
        state = queue.pop()
        its += 1
        if state.time_left <= 1:
            continue
        cur_flow = best[state.time_left][state.current][state.opened]

        if valves[state.current].rate != 0 and not(state.opened & (1 << ovalve_to_idx[state.current])):
            new_state = State(
                    current=state.current,
                    opened=state.opened | (1 << ovalve_to_idx[state.current]),
                    time_left=state.time_left - 1)
            new_flow = cur_flow + (state.time_left - 1) * valves[state.current].rate
            if best[new_state.time_left][new_state.current][new_state.opened] < new_flow:
                best[new_state.time_left][new_state.current][new_state.opened] = new_flow
                queue.append(new_state)

        for other in valves[state.current].other:
            new_state = State(
                    current=other,
                    opened=state.opened,
                    time_left=state.time_left - 1)
            new_flow = cur_flow

            if best[new_state.time_left][new_state.current][new_state.opened] < new_flow:
                best[new_state.time_left][new_state.current][new_state.opened] = new_flow
                queue.append(new_state)

    print(f'Total iterations: {its:,}')
    b = 0
    for opened in range(1 << idx):
        for val in range(len(valves)):
            b = max(b, best[1][val][opened])
    print(f'Best: {b}')

if __name__ == "__main__":
    main()
