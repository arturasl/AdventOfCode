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
        [f(c[i] for r in rocks for c in r) + o for f, o in [(min, mi), (max, ma)]]
        for i, mi, ma in [(0, 0, +2), (1, -200, +200)]
    ]
    ranges[0][0] = 0
    rocks.append([(ranges[0][1], ranges[1][0]), (ranges[0][1], ranges[1][1])])

    grid = [['.' for _ in range(ranges[1][1] - ranges[1][0] + 1)] for _ in range(ranges[0][1] - ranges[0][0] + 1)]

    def to_rel(s):
        return s[0] - ranges[0][0], s[1] - ranges[1][0]

    def get_abs(grid, s):
        s = to_rel(s)
        return grid[s[0]][s[1]]

    def set_abs(grid, s, c):
        s = to_rel(s)
        grid[s[0]][s[1]] = c

    for coords in rocks:
        for i in range(1, len(coords)):
            s = coords[i - 1]
            change = [sign(c) for c in add(coords[i], coords[i - 1], -1)]
            e = add(coords[i], change)
            while s != e:
                set_abs(grid, s, '#')
                s = add(s, change)

    path = [line[:] for line in grid]

    its = 0
    pos = (ranges[0][0], 500)
    set_abs(path, pos, 'v')
    while True:
        while True:
            if pos[0] == ranges[0][1]:
                break
            elif get_abs(grid, add(pos, [+1, 0])) == '.':
                pos = add(pos, [+1, 0])
                set_abs(path, pos, 'v')
            elif get_abs(grid, add(pos, [+1, -1])) == '.':
                pos = add(pos, [+1, -1])
                set_abs(path, pos, '↙')
            elif get_abs(grid, add(pos, [+1, +1])) == '.':
                pos = add(pos, [+1, +1])
                set_abs(path, pos, '↘')
            else:
                break
        if pos[0] == ranges[0][0]:
            break
        set_abs(grid, pos, 'o')
        its += 1

        if get_abs(path, pos) == 'v':
            pos = add(pos, [-1, 0])
        elif get_abs(path, pos) == '↙':
            pos = add(pos, [-1, +1])
        elif get_abs(path, pos) == '↘':
            pos = add(pos, [-1, -1])
        else:
            assert False

    print('\n'.join(''.join(line) for line in grid))
    print('')
    print('\n'.join(''.join(line) for line in path))
    print(its + 1)

if __name__ == "__main__":
    main()
