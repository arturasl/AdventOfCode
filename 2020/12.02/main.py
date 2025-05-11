from __future__ import annotations

import sys
from dataclasses import dataclass


@dataclass
class Coord:
    y: int
    x: int

    def __add__(self: Coord, other: Coord):
        return Coord(self.y + other.y, self.x + other.x)

    def __mul__(self: Coord, other: int):
        return Coord(self.y * other, self.x * other)


def main():
    waypoint = Coord(-1, +10)
    ship = Coord(0, 0)

    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        amount = int(line[1:])

        match line[0]:
            case "N":
                waypoint.y -= amount
            case "S":
                waypoint.y += amount
            case "E":
                waypoint.x += amount
            case "W":
                waypoint.x -= amount
            case "L" | "R":
                assert amount % 90 == 0
                for _ in range(amount // 90):
                    if line[0] == "L":
                        waypoint.x *= -1
                    else:
                        waypoint.y *= -1
                    waypoint.y, waypoint.x = waypoint.x, waypoint.y
            case "F":
                ship = ship + waypoint * amount
            case _:
                assert False

    print(abs(ship.y) + abs(ship.x))


if __name__ == "__main__":
    main()
