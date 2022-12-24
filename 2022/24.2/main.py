import sys
import re
from collections import namedtuple
from collections import defaultdict
from collections import deque

Wind = namedtuple('Wind', ['pos', 'dir'])

def read():
    grid = [list(line.strip()) for line in sys.stdin]

    assert sum(c == '#' for c in grid[0]) == len(grid[0]) - 1
    assert sum(c == '#' for c in grid[-1]) == len(grid[-1]) - 1
    for i in range(1, len(grid) - 1):
        assert grid[i][0] == '#' and grid[i][-1] == '#'
        assert len(set(grid[i][1:-1]) - set(list('.<>v^'))) == 0

    winds = set()
    for y in range(1, len(grid) - 1):
        for x in range(1, len(grid[y]) - 1):
            if grid[y][x] == '.':
                continue
            winds.add(Wind(
                pos=(y, x),
                dir={
                    '>': (0, +1),
                    '<': (0, -1),
                    '^': (-1, 0),
                    'v': (+1, 0),
                }[grid[y][x]]
            ))

    y_range = (1, len(grid) - 1)
    x_range = (1, len(grid[0]) - 1)

    def trim_dir(winds):
        return frozenset(w.pos for w in winds)

    winds = frozenset(winds)
    time_to_winds = [trim_dir(winds)]
    seen_winds = set([winds])
    while True:
        new_winds = set()
        for wind in winds:
            new_wind=Wind(
                pos=(
                    (wind.pos[0] + wind.dir[0] - y_range[0]) % (y_range[1] - y_range[0]) + y_range[0],
                    (wind.pos[1] + wind.dir[1] - x_range[0]) % (x_range[1] - x_range[0]) + x_range[0]),
                dir=wind.dir
            )
            new_winds.add(new_wind)

        if new_winds in seen_winds:
            break
        winds = frozenset(new_winds)
        time_to_winds.append(trim_dir(winds))
        seen_winds.add(winds)

    return (grid, time_to_winds)

def main():
    grid, time_to_winds = read()
    start = tuple((0, grid[0].index('.')))
    end = tuple((len(grid) - 1, grid[-1].index('.')))
    n = len(grid)
    m = len(grid[0])

    visited = [[
        [
            [
                x == 0 or x == m - 1 or (y == 0 and x != start[1]) or (y == n - 1 and x != end[1])
                for x in range(m)
            ] for y in range(n)
        ] for _ in range(len(time_to_winds))] for _ in range(3)]
    queue = deque([(start, 0, 0)])
    visited[0][0][start[0]][start[1]] = True

    while queue:
        cur_pos, t, s = queue.popleft()

        if s == 0 and cur_pos == end:
            s = 1
        elif s == 1 and cur_pos == start:
            s = 2
        elif s == 2 and cur_pos == end:
            print(t)
            break

        mod_t = (t + 1) % len(time_to_winds)
        t += 1

        for ny, nx in [(0, +1), (0, -1), (-1, 0), (+1, 0), (0, 0)]:
            next_pos = (cur_pos[0] + ny, cur_pos[1] + nx)
            if not (0 <= next_pos[0] < n):
                continue
            if not (0 <= next_pos[1] < m):
                continue
            if next_pos in time_to_winds[mod_t]:
                continue
            if visited[s][mod_t][next_pos[0]][next_pos[1]]:
                continue
            visited[s][mod_t][next_pos[0]][next_pos[1]] = True
            queue.append((next_pos, t, s))

if __name__ == "__main__":
    main()
