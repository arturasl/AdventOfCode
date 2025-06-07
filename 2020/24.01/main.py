import sys


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
    print(len(black))


if __name__ == "__main__":
    main()
