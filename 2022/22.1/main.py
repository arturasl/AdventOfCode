import sys
import re
import operator

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

def main():
    grid = [list(line.strip('\n')) for line in sys.stdin]
    moves = parse_moves(grid.pop())

    n, m = len(grid), max(len(row) for row in grid)
    grid = [row + [' '] * (m - len(row)) for row in grid]
    grid = [[' ' for _ in range(m)]] + grid
    n += 1
    grid = [[' '] + row + [' '] for row in grid]
    m += 2

    cur_pos = find_start(grid)
    direction = (0, +1)
    directions = [(0, +1), (+1, 0), (0, -1), (-1, 0)]
    dir_to_letter = {(0, +1): '>', (+1, 0): 'v', (0, -1): '<', (-1, 0): '^'}
    dir_to_facing = {(0, +1): 0, (+1, 0): 1, (0, -1): 2, (-1, 0): 3}

    print(moves)
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
                rev_direction = (-direction[0], -direction[1])
                wrap_pos = cur_pos
                result_pos = cur_pos
                while 0 <= wrap_pos[0] < len(grid) and 0 <= wrap_pos[1] < len(grid[0]):
                    if grid[wrap_pos[0]][wrap_pos[1]] != ' ':
                        result_pos = wrap_pos
                    wrap_pos = (wrap_pos[0] + rev_direction[0], wrap_pos[1] + rev_direction[1])
                if grid[result_pos[0]][result_pos[1]] == '#':
                    pass
                else:
                    cur_pos = result_pos

    print('\n'.join(''.join(row) for row in grid))
    print(cur_pos[0] * 1_000 + cur_pos[1] * 4 + dir_to_facing[direction])

if __name__ == "__main__":
    main()
