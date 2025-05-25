from __future__ import annotations

import itertools
import sys
from collections import defaultdict

AROUND = [
    oft
    for oft in itertools.product(list(range(-1, 1 + 1)), repeat=4)
    if any(x != 0 for x in oft)
]

COORD = tuple[int, int, int, int]


def read_state() -> set[COORD]:
    active: set[COORD] = set()
    y = 0
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        for x, c in enumerate(line):
            assert c in "#."
            if c == "#":
                active.add(COORD([y, x, 0, 0]))
        y += 1
    return active


def main():
    active = read_state()
    for _ in range(6):
        active_around: dict[COORD, int] = defaultdict(int)
        for coord in active:
            for oft in AROUND:
                active_around[COORD(a + b for a, b in zip(coord, oft))] += 1

        next_active: set[COORD] = set()
        for coord, num_active in active_around.items():
            if num_active == 3 or (num_active == 2 and coord in active):
                next_active.add(coord)
        active = next_active

    print(len(active))


if __name__ == "__main__":
    main()
