import itertools
import sys
from collections import deque


def play(p1: deque[int], p2: deque[int], cur_cache: set[str]) -> bool:
    if not p1:
        return False
    if not p2:
        return True

    p1_is_winner = False

    cache_key = "|".join(",".join(str(x) for x in p) for p in [p1, p2])
    if cache_key in cur_cache:
        p1_is_winner = True
        return p1_is_winner
    cur_cache.add(cache_key)

    p1_num, p2_num = p1.popleft(), p2.popleft()
    assert p1_num != p2_num

    if len(p1) >= p1_num and len(p2) >= p2_num:
        p1_is_winner = play(deque(list(p1)[:p1_num]), deque(list(p2)[:p2_num]), set())
    else:
        p1_is_winner = p1_num > p2_num

    if p1_is_winner:
        p1.extend([p1_num, p2_num])
    else:
        p2.extend([p2_num, p1_num])

    return play(p1, p2, cur_cache)


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

    _ = play(p1, p2, set())

    print(
        sum(
            sum(a * b for a, b in zip(list(p)[::-1], itertools.count(start=1)))
            for p in [p1, p2]
        )
    )


if __name__ == "__main__":
    sys.setrecursionlimit(10_000)
    main()
