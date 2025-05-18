import sys
from dataclasses import dataclass
from functools import reduce


@dataclass
class Bus:
    oft: int
    bus_id: int


def ext_gcd(a: int, b: int):
    matrix = [[1, 0, a], [0, 1, b]]
    while matrix[1][2]:
        div = matrix[0][2] // matrix[1][2]
        for i in range(3):
            matrix[0][i] -= matrix[1][i] * div
        matrix[0], matrix[1] = matrix[1], matrix[0]
    assert matrix[0][2] == 1
    return matrix[0]


def main():
    lines = [line for line in [line.strip() for line in sys.stdin] if line]
    busses = [int(bus) if bus != "x" else None for bus in lines[1].split(",")]
    busses = [Bus(bus - oft, bus) for oft, bus in enumerate(busses) if bus is not None]

    prod = reduce(lambda x, y: x * y, (bus.bus_id for bus in busses), 1)
    ans = 0
    for bus in busses:
        n = prod // bus.bus_id
        ans += ext_gcd(n, bus.bus_id)[0] * bus.oft * n
    print(ans % prod)


if __name__ == "__main__":
    main()
