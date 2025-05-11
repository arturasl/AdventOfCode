import sys
from dataclasses import dataclass


@dataclass
class Coord:
    y: int
    x: int


@dataclass
class Dir:
    dy: int
    dx: int


def main():
    cur_dir = Dir(0, +1)
    cur_pos = Coord(0, 0)

    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        amount = int(line[1:])

        match line[0]:
            case "N":
                cur_pos.y -= amount
            case "S":
                cur_pos.y += amount
            case "E":
                cur_pos.x += amount
            case "W":
                cur_pos.x -= amount
            case "L":
                assert amount % 90 == 0
                for _ in range(amount // 90):
                    cur_dir.dx *= -1
                    cur_dir.dy, cur_dir.dx = cur_dir.dx, cur_dir.dy
            case "R":
                assert amount % 90 == 0
                for _ in range(amount // 90):
                    cur_dir.dy *= -1
                    cur_dir.dy, cur_dir.dx = cur_dir.dx, cur_dir.dy
            case "F":
                cur_pos.y += cur_dir.dy * amount
                cur_pos.x += cur_dir.dx * amount
            case _:
                assert False

    print(abs(cur_pos.y) + abs(cur_pos.x))


if __name__ == "__main__":
    main()
