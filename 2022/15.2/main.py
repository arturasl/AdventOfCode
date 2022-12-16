from z3 import *
import sys
import re

def Abs(x):
    return If(x >= 0, x, -x)

def main():
    max_c = 4_000_000

    x, y = Ints('x y')
    s = Solver()

    s.add(0 <= x)
    s.add(x <= max_c)
    s.add(0 <= y)
    s.add(y <= max_c)


    poses = []
    for line in [line.strip() for line in sys.stdin if line]:
        m = re.match(r'^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$', line)
        assert m
        m = [int(x) for x in m.groups()]
        poses.append({'sensor': (m[0], m[1]), 'beacon': (m[2], m[3])})

    for pos in poses:
        dist = abs(pos['sensor'][0] - pos['beacon'][0]) + abs(pos['sensor'][1] - pos['beacon'][1]) + 1
        s.add(Abs(pos['sensor'][0] - x) + Abs(pos['sensor'][1] - y) + 1 > dist)

    if s.check() == sat:
        m = s.model()
        print(m)
        print(m[x].as_long() * max_c + m[y].as_long())
    else:
        print(':(')

if __name__ == "__main__":
    main()
