import sys
from collections import defaultdict
import copy

def sign(x):
    if x < 0:
        return -1
    if x > 0:
        return +1
    return 0

def main():
    dirs = [(a, int(b)) for line in sys.stdin for (a, b) in [line.strip().split()] if line]
    max_paths = defaultdict(int)
    for d, a in dirs:
        max_paths[d] += a
    dim = max(max_paths.values())
    n = dim * 2 + 1
    grid = [[0 for _ in range(n)] for _ in range(n)]
    chain = [[0, 0] for _ in range(10)]

    def to_grid(p):
        return [p[0] + dim, p[1] + dim]
    def trans(d):
        return {'L': (0, -1), 'R': (0, +1), 'D': (+1, 0), 'U': (-1, 0)}[d]

    grid[to_grid(chain[9])[0]][to_grid(chain[9])[1]] = 1
    for d, a in dirs:
        # print(f'Direction: {d}, Amount: {a}')

        for _ in range(a):
            chain[0][0] += trans(d)[0]
            chain[0][1] += trans(d)[1]
            for i in range(1, 10, 1):
                dy = chain[i - 1][0] - chain[i][0]
                dx = chain[i - 1][1] - chain[i][1]
                if abs(dy) >= 2:
                    chain[i][0] += dy - sign(dy)
                if abs(dx) >= 2:
                    chain[i][1] += dx - sign(dx)
                if abs(dy) >= 2 and abs(dx) == 1:
                    chain[i][1] += dx
                if abs(dy) == 1 and abs(dx) >= 2:
                    chain[i][0] += dy
            grid[to_grid(chain[9])[0]][to_grid(chain[9])[1]] = 1

        # for i in range(n):
        #     for j in range(n):
        #         for x, (a, b) in enumerate(chain):
        #             if to_grid([a, b]) == [i, j]:
        #                 print('H' if x == 0 else str(x), end='')
        #                 break
        #         else:
        #             print('_' if grid[i][j] == 0 else '#', end='')
        #     print('')
        # print('')

    print(sum(sum(x) for x in grid))

if __name__ == "__main__":
    main()
