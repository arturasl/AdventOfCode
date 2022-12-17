import sys

def can_draw(wall, pattern, off, draw=False):
    for py in range(len(pattern)):
        for px, ch in enumerate(pattern[py]):
            wy = len(wall) - off[0] - py - 1
            wx = px + off[1]
            if wy <= -1 or wx <= -1 or wx >= len(wall[wy]):
                return False
            if pattern[py][px] != '.':
                if wall[wy][wx] != '.':
                    return False
                if draw:
                    wall[wy][wx] = '#'
    return True

def add(l, r):
    return (l[0] + r[0], l[1] + r[1])

def show(wall):
    wa = [w[:] for w in wall]
    print('\n'.join(''.join(w) for w in wa[::-1]))
    print('')

def append(wall, pat, movements, mov):
    for _ in range(3 + len(pat)):
        wall.append(['.'] * 7)
    off = (0, 2)

    while True:
        x_dir = (0, +1) if movements[mov] == '>' else (0, -1)
        if can_draw(wall, pat, add(off, x_dir)):
            off = add(off, x_dir)
        mov = (mov + 1) % len(movements)

        if not can_draw(wall, pat, add(off, (+1, 0))):
            break
        off = add(off, (+1, 0))

    assert can_draw(wall, pat, off, draw=True)

    while wall and wall[-1] == ['.'] * 7:
        wall.pop()

    return mov

def main():
    movements = list(next(iter(sys.stdin)).strip())
    patterns = '''
    ####

    .#.
    ###
    .#.

    ..#
    ..#
    ###

    #
    #
    #
    #

    ##
    ##
    '''
    patterns = [[l.strip() for l in p.strip().split('\n')] for p in patterns.split('\n\n')]

    result = 0

    wait_till_p = 2022
    wall = []
    p = 0
    mov = 0

    while p < wait_till_p:
        mov = append(wall, patterns[p % len(patterns)], movements, mov)
        p = p + 1

    show(wall)
    print(len(wall))

if __name__ == "__main__":
    main()
