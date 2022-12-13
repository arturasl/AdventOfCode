import sys
from collections import defaultdict

def main():
    dirs = [(a, int(b)) for line in sys.stdin for (a, b) in [line.strip().split()] if line]
    max_paths = defaultdict(int)
    for d, a in dirs:
        max_paths[d] += a
    dim = max(max_paths.values())
    n = dim * 2 + 1
    grid = [[0 for _ in range(n)] for _ in range(n)]
    h_pos, t_pos = [0, 0], [0, 0]
    def to_grid(p):
        return [p[0] + dim, p[1] + dim]
    def trans(d):
        return {'L': (0, -1), 'R': (0, +1), 'D': (+1, 0), 'U': (-1, 0)}[d]

    grid[to_grid(t_pos)[0]][to_grid(t_pos)[1]] = 1
    for d, a in dirs:
        print(f'Direction: {d}, Amount: {a}')

        for _ in range(a):
            h_pos[0] += trans(d)[0]
            h_pos[1] += trans(d)[1]
            if abs(h_pos[0] - t_pos[0]) >= 2 or abs(h_pos[1] - t_pos[1]) >= 2:
                t_pos[0] = h_pos[0] - trans(d)[0]
                t_pos[1] = h_pos[1] - trans(d)[1]
                grid[to_grid(t_pos)[0]][to_grid(t_pos)[1]] = 1

            # for i in range(n):
            #     print(''.join(
            #         'H' if to_grid(h_pos) == [i, j]
            #         else 'T' if to_grid(t_pos) == [i, j]
            #         else '_' if grid[i][j] == 0
            #         else '#'
            #         for j in range(n)))
            # print('')

    print(sum(sum(x) for x in grid))

if __name__ == "__main__":
    main()
