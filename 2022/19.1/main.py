import sys
import re
from collections import defaultdict
from collections import namedtuple
from heapq import *

Blueprint = namedtuple('Blueprint', ['idx', 'cost_per_type'])

def read_blueprints():
    it = iter(sys.stdin)
    def read_re(regexp):
        line = ''
        while not line:
            line = next(it).strip()
        m = re.match(regexp, line)
        assert m, line
        return m.groups()

    blueprints = []
    while True:
        try:
            m = read_re('^Blueprint (\d+):?$')
        except StopIteration:
            break
        assert len(blueprints) + 1 == int(m[0])

        blueprint = Blueprint(idx=len(blueprints), cost_per_type={})

        for _ in range(4):
            m = read_re('^Each (\w+) robot costs ((?:(?: and )?(?:\d+ \w+))+).?$')
            costs = {name: int(cost) for cost, name in [x.split() for x in m[1].split(' and ')]}
            assert costs
            assert m[0] not in blueprint.cost_per_type
            blueprint.cost_per_type[m[0]] = costs

        blueprints.append(blueprint)

    return blueprints

def mul(items):
    r = 1
    for i in items:
        r *= max(1, i)
    return r

class State():
    def __init__(self, s):
        self.s = s

    def __lt__(self, other):
        return self.s[2] > other.s[2]


def solve_blueprint(blueprint):
    ordered_types = ['ore', 'clay', 'obsidian', 'geode']
    blueprint = [
        [blueprint.cost_per_type[item][subitem] if subitem in blueprint.cost_per_type[item] else 0
         for subitem in ordered_types]
        for item in ordered_types
    ]
    print(f'blueprint: {blueprint}')

    max_items = [max(blueprint[j][i] for j in range(len(blueprint))) for i in range(len(blueprint[0]))]
    print(f'max_items: {max_items}')

    states = defaultdict(lambda: defaultdict(lambda: tuple((-1, -1))))
    queue = [State(((1, 0, 0, 0), (0, 0, 0), (0, 24)))]
    its = 0
    while queue:
        state = heappop(queue).s
        robots = list(state[0])
        items = list(state[1]) + [state[2][0]]
        time_left = state[2][1]

        its += 1
        if its % 1_000_000 == 0:
            print(its, max(mul(s0) * mul(s1) for s0, v in states.items() for s1 in v ))

        if states[state[0]][state[1]] >= state[2]:
            continue
        states[state[0]][state[1]] = state[2]
        if time_left == 0:
            continue

        if not any(all(item >= 2 * cost for item, cost in zip(items[:-1], costs[:-1])) for costs in blueprint):
            new_items = [i + r for i, r in zip(items, robots)]
            heappush(queue, State((tuple(robots), tuple(new_items[:-1]), (new_items[-1], time_left - 1))))

        for b, costs in enumerate(blueprint):
            new_items = items[:]
            for item, cost in enumerate(costs):
                new_items[item] -= cost
            if any(i < 0 for i in new_items):
                continue
            new_items = [i + r for i, r in zip(new_items, robots)]
            new_robots = robots[:]
            new_robots[b] += 1
            heappush(queue, State((tuple(new_robots), tuple(new_items[:-1]), (new_items[-1], time_left - 1))))


    print(f'total its: {its:,}')

    return max(v1[0] for s0, v0 in states.items() for s1, v1 in v0.items())

def main():
    blueprints = read_blueprints()
    r = 0
    for blueprint in blueprints:
        s = solve_blueprint(blueprint)
        print(f'{blueprint.idx}: {s}')
        r += (blueprint.idx + 1) * s
    print(f'Result: {r}')

if __name__ == "__main__":
    main()
