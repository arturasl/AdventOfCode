import sys

def sign(x):
    if x < 0:
        return -1
    if x > 0:
        return +1
    return 0

def add(lhs, rhs, s = 1):
    return [l + s * r for l, r in zip(lhs, rhs)]

def main():
    rocks = []
    for line in [line.strip().replace(' ', '') for line in sys.stdin]:
        rocks.append([[int(c) for c in pair.split(',')[::-1]] for pair in line.split('->')])

    ranges = [
        [f(c[i] for r in rocks for c in r) + o for f, o in [(min, -3), (max, +1)]]
        for i in [0, 1]
    ]

    grid = [['.' for _ in range(ranges[1][1] - ranges[1][0] + 1)] for _ in range(ranges[0][1] - ranges[0][0] + 1)]

    def to_rel(s):
        return s[0] - ranges[0][0], s[1] - ranges[1][0]

    def get_abs(s):
        s = to_rel(s)
        return grid[s[0]][s[1]]

    def set_abs(s, c):
        s = to_rel(s)
        grid[s[0]][s[1]] = c

    for coords in rocks:
        for i in range(1, len(coords)):
            s = coords[i - 1]
            change = [sign(c) for c in add(coords[i], coords[i - 1], -1)]
            e = add(coords[i], change)
            while s != e:
                set_abs(s, '#')
                s = add(s, change)

    its = 0
    while True:
        pos = (ranges[0][0], 500)
        while True:
            if pos[0] == ranges[0][1]:
                pos = None
                break
            elif get_abs(add(pos, [+1, 0])) == '.':
                pos = add(pos, [+1, 0])
            elif get_abs(add(pos, [+1, -1])) == '.':
                pos = add(pos, [+1, -1])
            elif get_abs(add(pos, [+1, +1])) == '.':
                pos = add(pos, [+1, +1])
            else:
                break
        if not pos:
            break
        set_abs(pos, 'o')
        its += 1

    print('\n'.join(''.join(line) for line in grid))
    print(its)

if __name__ == "__main__":
    main()
