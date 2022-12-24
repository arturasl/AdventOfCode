import sys
import re
import operator
from collections import namedtuple

DIR_UP = (-1, 0)
DIR_DOWN = (+1, 0)
DIR_LEFT = (0, -1)
DIR_RIGHT = (0, +1)
DIM = 50

def parse_moves(dirs):
    str_number = ''
    parsed = []

    for c in dirs:
        if '0' <= c <= '9':
            str_number += c
        else:
            if str_number:
                parsed.append(int(str_number))
                str_number = ''
            parsed.append(c)

    if str_number:
        parsed.append(int(str_number))

    return parsed

def find_start(grid):
    start = (-1, -1)
    for y, row in enumerate(grid):
        if start[0] != -1:
            break
        for x, c in enumerate(row):
            if c == '.':
                start = (y, x)
                break
    assert start[0] != -1 and start[1] != -1
    return start

def show_grid(grid):
    print('\n'.join(''.join(str(s) for s in row) for row in grid))

def get_cube_enc(n, m):
    Encoding = namedtuple('Encoding', ['direction', 'y_range', 'x_range', 'next'])
    encodings = {}

    # encodings['L'] = Encoding(direction=DIR_UP, y_range=DIM * 0, x_range=DIM * 2, next=('K', -1))
    # encodings['K'] = Encoding(direction=DIR_UP, y_range=DIM * 1, x_range=DIM * 0, next=('L', -1))
    # encodings['J'] = Encoding(direction=DIR_UP, y_range=DIM * 1, x_range=DIM * 1, next=('I', +1))
    # encodings['B'] = Encoding(direction=DIR_UP, y_range=DIM * 2, x_range=DIM * 3, next=('A', -1))
    #
    # encodings['M'] = Encoding(direction=DIR_RIGHT, y_range=DIM * 0, x_range=DIM * 3 - 1, next=('N', -1))
    # encodings['A'] = Encoding(direction=DIR_RIGHT, y_range=DIM * 1, x_range=DIM * 3 - 1, next=('B', -1))
    # encodings['N'] = Encoding(direction=DIR_RIGHT, y_range=DIM * 2, x_range=DIM * 4 - 1, next=('M', -1))
    #
    # encodings['I'] = Encoding(direction=DIR_LEFT, y_range=DIM * 0, x_range=DIM * 2, next=('J', +1))
    # encodings['F'] = Encoding(direction=DIR_LEFT, y_range=DIM * 1, x_range=DIM * 0, next=('E', -1))
    # encodings['H'] = Encoding(direction=DIR_LEFT, y_range=DIM * 2, x_range=DIM * 2, next=('G', -1))
    #
    # encodings['D'] = Encoding(direction=DIR_DOWN, y_range=DIM * 2 - 1, x_range=DIM * 0, next=('C', -1))
    # encodings['G'] = Encoding(direction=DIR_DOWN, y_range=DIM * 2 - 1, x_range=DIM * 1, next=('H', -1))
    # encodings['C'] = Encoding(direction=DIR_DOWN, y_range=DIM * 3 - 1, x_range=DIM * 2, next=('D', -1))
    # encodings['E'] = Encoding(direction=DIR_DOWN, y_range=DIM * 3 - 1, x_range=DIM * 3, next=('F', -1))

    #################

    encodings['A'] = Encoding(direction=DIR_UP, y_range=DIM * 0, x_range=DIM * 1, next=('J', +1))
    encodings['B'] = Encoding(direction=DIR_UP, y_range=DIM * 0, x_range=DIM * 2, next=('I', +1))
    encodings['L'] = Encoding(direction=DIR_UP, y_range=DIM * 2, x_range=DIM * 0, next=('M', +1))

    encodings['C'] = Encoding(direction=DIR_RIGHT, y_range=DIM * 0, x_range=DIM * 3 - 1, next=('F', -1))
    encodings['E'] = Encoding(direction=DIR_RIGHT, y_range=DIM * 1, x_range=DIM * 2 - 1, next=('D', +1))
    encodings['F'] = Encoding(direction=DIR_RIGHT, y_range=DIM * 2, x_range=DIM * 2 - 1, next=('C', -1))
    encodings['H'] = Encoding(direction=DIR_RIGHT, y_range=DIM * 3, x_range=DIM * 1 - 1, next=('G', +1))

    encodings['N'] = Encoding(direction=DIR_LEFT, y_range=DIM * 0, x_range=DIM * 1, next=('K', -1))
    encodings['M'] = Encoding(direction=DIR_LEFT, y_range=DIM * 1, x_range=DIM * 1, next=('L', +1))
    encodings['K'] = Encoding(direction=DIR_LEFT, y_range=DIM * 2, x_range=DIM * 0, next=('N', -1))
    encodings['J'] = Encoding(direction=DIR_LEFT, y_range=DIM * 3, x_range=DIM * 0, next=('A', +1))

    encodings['D'] = Encoding(direction=DIR_DOWN, y_range=DIM * 1 - 1, x_range=DIM * 2, next=('E', +1))
    encodings['G'] = Encoding(direction=DIR_DOWN, y_range=DIM * 3 - 1, x_range=DIM * 1, next=('H', +1))
    encodings['I'] = Encoding(direction=DIR_DOWN, y_range=DIM * 4 - 1, x_range=DIM * 0, next=('B', +1))

    for f in encodings.keys():
        enc = encodings[f]
        y_range, x_range = None, None
        if enc.direction in [DIR_UP, DIR_DOWN]:
            y_range = enc.y_range + 1, enc.y_range + 2
            x_range = enc.x_range + 1, enc.x_range + DIM + 1
        elif enc.direction in [DIR_LEFT, DIR_RIGHT]:
            y_range = enc.y_range + 1, enc.y_range + DIM + 1
            x_range = enc.x_range + 1, enc.x_range + 2
        encodings[f] = Encoding(
            direction=enc.direction,
            y_range=y_range,
            x_range=x_range,
            next=enc.next
        )

    assert len(encodings.keys()) == 14
    assert all(l in encodings for l in 'ABCDEFGHIJKLMN')
    for f, enc in encodings.items():
        assert encodings[enc.next[0]].next == (f, enc.next[1]), enc

    return encodings

def show_enc(encodings, direction, grid):
    for f, e in encodings.items():
        print(f, e)
    grid = [row[:] for row in grid]
    i = 1
    for enc in encodings.values():
        if enc.direction != direction:
            continue
        for y in range(enc.y_range[0], enc.y_range[1]):
            for x in range(enc.x_range[0], enc.x_range[1]):
                grid[y][x] = i
        i += 1
    show_grid(grid)

def get_next(y, x, direction, encodings):
    enc = None
    for other_enc in encodings.values():
        if direction != other_enc.direction:
            continue
        if not (other_enc.y_range[0] <= y < other_enc.y_range[1]):
            continue
        if not (other_enc.x_range[0] <= x < other_enc.x_range[1]):
            continue
        assert enc is None
        enc = other_enc

    assert enc is not None

    next_enc = encodings[enc.next[0]]
    next_direction = next_enc.direction
    next_direction = (-next_direction[0], -next_direction[1])

    off = None
    if enc.y_range[1] - enc.y_range[0] == 1:
        off = x - enc.x_range[0]
    else:
        off = y - enc.y_range[0]
    assert 0 <= off < DIM

    if enc.next[1] == -1:
        off = DIM - off - 1

    if next_enc.y_range[1] - next_enc.y_range[0] == 1:
        return ((next_enc.y_range[0], next_enc.x_range[0] + off), next_direction)
    else:
        return ((next_enc.y_range[0] + off, next_enc.x_range[0]), next_direction)

def main():
    grid = [list(line.strip('\n')) for line in sys.stdin]
    moves = parse_moves(grid.pop())

    n, m = len(grid), max(len(row) for row in grid)
    grid = [row + [' '] * (m - len(row)) for row in grid]
    grid = [[' ' for _ in range(m)]] + grid
    n += 1
    grid = [[' '] + row + [' '] for row in grid]
    m += 2

    # show_grid(grid)
    encs = get_cube_enc(n, m)
    # show_enc(encs, DIR_DOWN, grid)
    # return 0

    cur_pos = find_start(grid)
    direction = DIR_RIGHT

    directions = [DIR_RIGHT, DIR_DOWN, DIR_LEFT, DIR_UP]
    dir_to_letter = {DIR_RIGHT: '>', DIR_DOWN: 'v', DIR_LEFT: '<', DIR_UP: '^'}
    dir_to_facing = {DIR_RIGHT: 0, DIR_DOWN: 1, DIR_LEFT: 2, DIR_UP: 3}

    for move in moves:
        if type(move) == type('c'):
            next_dir = directions.index(direction) + (+1 if move == 'R' else -1)
            direction = directions[next_dir % len(directions)]
            continue
        for i in range(move):
            grid[cur_pos[0]][cur_pos[1]] = dir_to_letter[direction]
            next_pos = cur_pos[0] + direction[0], cur_pos[1] + direction[1]
            if grid[next_pos[0]][next_pos[1]] not in ['#', ' ']:
                cur_pos = next_pos
            elif grid[next_pos[0]][next_pos[1]] == '#':
                pass
            elif grid[next_pos[0]][next_pos[1]] == ' ':
                wrap_pos, wrap_direction = get_next(cur_pos[0], cur_pos[1], direction, encs)
                if grid[wrap_pos[0]][wrap_pos[1]] == '#':
                    pass
                else:
                    cur_pos = wrap_pos
                    direction = wrap_direction
    grid[cur_pos[0]][cur_pos[1]] = 'x'

    show_grid(grid)
    print(cur_pos[0] * 1_000 + cur_pos[1] * 4 + dir_to_facing[direction])

if __name__ == "__main__":
    main()
