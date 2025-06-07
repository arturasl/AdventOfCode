import sys
from collections import defaultdict


def calc_delta_x(d: str) -> int:
    match d:
        case "e":
            return 1
        case "w":
            return -1
        case _:
            assert False


def walk(path: str) -> tuple[int, int]:
    y, x = 0, 0
    i = 0
    while i != len(path):
        if path[i] in "ew":
            x += calc_delta_x(path[i]) * 2
        else:
            assert path[i] in "sn"
            y += 1 if path[i] == "s" else -1
            i += 1
            x += calc_delta_x(path[i])
        i += 1

    return (y, x)


def test_close_to_ref():
    assert walk("esew") == (1, 1)


def test_back_to_reference():
    assert walk("nwwswee") == (0, 0)


def main():
    black: set[tuple[int, int]] = set()
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        moved = walk(line)
        if moved in black:
            black.remove(moved)
        else:
            black.add(moved)

    deltas = [walk(d) for d in ["e", "se", "sw", "w", "nw", "ne"]]

    for _ in range(100):
        adj: dict[tuple[int, int], int] = defaultdict(int)
        for y, x in black:
            for delta in deltas:
                adj[(y + delta[0], x + delta[1])] += 1

        next_black: set[tuple[int, int]] = set()
        for (y, x), adj_black in adj.items():
            is_black = (y, x) in black
            if adj_black in (is_black, 2):
                next_black.add((y, x))
        black = next_black
    print(len(black))


if __name__ == "__main__":
    main()
