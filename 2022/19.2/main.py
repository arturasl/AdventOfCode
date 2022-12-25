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
        return self.s[3] > other.s[3]


def solve_blueprint(blueprint):
    ordered_types = ['ore', 'clay', 'obsidian', 'geode']
    blueprint = [
        [blueprint.cost_per_type[item][subitem] if subitem in blueprint.cost_per_type[item] else 0
         for subitem in ordered_types]
        for item in ordered_types
    ]
    print(f'blueprint: {blueprint}')

    # max_time = 24
    max_time = 32

    max_robots = [max(blueprint[j][i] for j in range(len(blueprint))) for i in range(len(blueprint[0]))]
    max_robots[-1] = max_time
    print(f'max_robots: {max_robots}')

    states = defaultdict(lambda: defaultdict(lambda: defaultdict(lambda: -1)))
    queue = [State(((1, 0, 0, 0), (0, 0, 0), max_time, 0, ()))]

    def clamp(new_robots, new_items, time_left):
        new_robots = [min(a, b) for a, b in zip(new_robots, max_robots)]
        return new_robots, new_items

    its = 0
    best_so_far = 0
    while queue:
        state = heappop(queue).s
        robots = list(state[0])
        items = list(state[1]) + [state[3]]
        time_left = state[2]
        could_have_bought = state[4]

        its += 1
        if its % 1_000_000 == 0:
            print(f'{its:,} {best_so_far:,}')

        if states[state[0]][state[1]][time_left] >= items[-1]:
            continue
        states[state[0]][state[1]][time_left] = items[-1]

        best_so_far = max(best_so_far, items[-1])

        if time_left == 0:
            continue

        cur = items[-1]
        for i in range(time_left):
            cur += robots[-1] + i
        if cur < best_so_far:
            continue

        can_buy_all = True
        new_could_have_bought = []
        for b, costs in enumerate(blueprint):
            new_items = items[:]
            for item, cost in enumerate(costs):
                new_items[item] -= cost
            if any(i < 0 for i in new_items):
                can_buy_all = False
                continue
            new_could_have_bought.append(b)
            if b in could_have_bought:
                continue
            new_items = [i + r for i, r in zip(new_items, robots)]
            new_robots = robots[:]
            new_robots[b] += 1
            new_robots, new_items = clamp(new_robots, new_items, time_left)
            heappush(queue, State((tuple(new_robots), tuple(new_items[:-1]), time_left - 1, new_items[-1], ())))

        new_items = [i + r for i, r in zip(items, robots)]
        heappush(queue, State((tuple(robots), tuple(new_items[:-1]), time_left - 1, new_items[-1], new_could_have_bought)))


    print(f'total its: {its:,}')
    return best_so_far

def main():
    blueprints = read_blueprints()
    r1 = 0
    r2 = 1
    for blueprint in blueprints:
        s = solve_blueprint(blueprint)
        print(f'{blueprint.idx}: {s}')
        r1 += (blueprint.idx + 1) * s
        r2 *= s
    print(f'Results. 1: {r1}, 2: {r2}')

if __name__ == "__main__":
    main()
