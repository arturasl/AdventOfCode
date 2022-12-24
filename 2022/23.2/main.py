import sys
import re
from collections import namedtuple
from collections import defaultdict

Pos = namedtuple('Pos', ['y', 'x'])

D_N = Pos(y=-1, x=0)
D_E = Pos(y=0, x=+1)
D_S = Pos(y=+1, x=0)
D_W = Pos(y=0, x=-1)

def MakePos(l):
    if type(l) == type(Pos(0, 0)):
        return l
    return Pos(y=l[0], x=l[1])

def Add(l, r):
    l = MakePos(l)
    r = MakePos(r)
    return Pos(y=l.y + r.y, x=l.x + r.x)

def PrintElfs(elfs):
    y_range = [f(elf.y for elf in elfs) for f in [min, max]]
    x_range = [f(elf.x for elf in elfs) for f in [min, max]]

    for y in range(y_range[0], y_range[1] + 1):
        for x in range(x_range[0], x_range[1] + 1):
            print('#' if MakePos((y, x)) in elfs else '.', end='')
        print('')
    print(f'Empty: {(y_range[1] - y_range[0] + 1) * (x_range[1] - x_range[0] + 1) - len(elfs)}')
    print('')

consider = [
    (D_N, [D_N, Add(D_N, D_E), Add(D_N, D_W)]),
    (D_S, [D_S, Add(D_S, D_E), Add(D_S, D_W)]),
    (D_W, [D_W, Add(D_W, D_N), Add(D_W, D_S)]),
    (D_E, [D_E, Add(D_E, D_N), Add(D_E, D_S)]),
]

def get_next(elf, elfs, r):
    nothing = True
    for y in [-1, 0, 1]:
        for x in [-1, 0, 1]:
            if y == 0 and x == 0:
                continue
            if Add(elf, (y, x)) in elfs:
                nothing = False
    if nothing:
        return elf

    for i in range(len(consider)):
        nothing = True
        c = consider[(i + r) % len(consider)]
        for n in c[1]:
            if Add(elf, n) in elfs:
                nothing = False
        if nothing:
            return Add(elf, c[0])

    return elf

def check(p, poses):
    n = Add(poses[Add(p, (-1, -1))], (+1, +1))
    print(f'{p} -> {n}')

def main():
    elfs = set([])
    for y, line in enumerate(sys.stdin):
        for x, c in enumerate(line.strip()):
            if c == '#':
                elfs.add(Pos(y=y, x=x))

    r = 0
    while True:
        print(f'# Round {r + 1}')

        new_poses = {}
        for elf in elfs:
            new_poses[elf] = get_next(elf, elfs, r)
        assert len(new_poses) == len(elfs)

        # check(MakePos((3, 8)), new_poses)
        # check(MakePos((4, 6)), new_poses)
        # check(MakePos((4, 10)), new_poses)

        poisoned = defaultdict(int)
        for pos in new_poses.values():
            poisoned[pos] += 1

        new_elfs = set()
        for prev_pos, new_pos in new_poses.items():
            pos = new_pos
            if poisoned[new_pos] != 1:
                pos = prev_pos
            assert pos not in new_elfs
            new_elfs.add(pos)
        assert len(new_poses) == len(new_elfs)

        if new_elfs == elfs:
            break

        elfs = new_elfs
        r += 1

    PrintElfs(elfs)

if __name__ == "__main__":
    main()
