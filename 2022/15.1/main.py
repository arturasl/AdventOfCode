import sys
import re

def main():
    poses = []
    for line in [line.strip() for line in sys.stdin if line]:
        m = re.match(r'^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$', line)
        assert m
        m = [int(x) for x in m.groups()]
        poses.append({'sensor': (m[0], m[1]), 'beacon': (m[2], m[3])})

    ints = []
    line_y = 2_000_000
    for pos in poses:
        dist = abs(pos['sensor'][0] - pos['beacon'][0]) + abs(pos['sensor'][1] - pos['beacon'][1]) + 1
        dist_ln = abs(pos['sensor'][1] - line_y) + 1
        left = dist - dist_ln
        if left >= 0:
            ints.append((pos['sensor'][0] - left, pos['sensor'][0] + left + 1))

    ints = sorted(ints, key=lambda x: (x[0], -x[1]))
    print(ints)

    new_ints = []
    i = 0
    while i < len(ints):
        j = i
        e = ints[i][1]
        while j < len(ints) and e >= ints[j][0]:
            e = max([e, ints[j][1]])
            j += 1
        new_ints.append((ints[i][0], e))
        i = j

    print(new_ints)

    new_int_ln = sum([x[1] - x[0] for x in new_ints])
    num_beacons_on_line = len(set([b['beacon'][0] for b in poses if b['beacon'][1] == line_y]))
    print(new_int_ln - num_beacons_on_line)

if __name__ == "__main__":
    main()
