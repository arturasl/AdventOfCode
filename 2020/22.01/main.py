import itertools
import sys
from collections import deque


def main():
    p1: deque[int] = deque()
    p2: deque[int] = deque()
    filling: deque[int] | None = None

    for line in sys.stdin:
        line = line.strip()

        if not line:
            continue

        if line == "Player 1:":
            assert filling is None
            filling = p1
        elif line == "Player 2:":
            filling = p2
        else:
            assert filling is not None
            filling.append(int(line))

    while p1 and p2:
        p1_num, p2_num = p1.popleft(), p2.popleft()

        if p1_num > p2_num:
            p1.extend([p1_num, p2_num])
        elif p1_num < p2_num:
            p2.extend([p2_num, p1_num])
        else:
            assert False

    print(
        sum(
            sum(a * b for a, b in zip(list(p)[::-1], itertools.count(start=1)))
            for p in [p1, p2]
        )
    )


if __name__ == "__main__":
    main()
