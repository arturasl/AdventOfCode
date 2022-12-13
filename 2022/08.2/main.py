import sys

def rot(grid):
    n = len(grid)
    new_grid = [[None for _ in range(n)] for _ in range(n)]
    for i in range(n):
        for j in range(n):
            new_grid[j][n - i - 1] = grid[i][j]
    return new_grid

def show(grid):
    for row in grid:
        print(' '.join(str(int(r)) for r in row))
    print('')

def main():
    grid = [[int(ch) for ch in line.strip()] for line in sys.stdin]
    assert len(grid) == len(grid[0])
    n = len(grid)

    ans = [[1 for _ in range(n)] for _ in range(n)]

    for r in range(4):
        for i in range(n):
            mask = [[0 for _ in range(9 + 1)] for _ in range(n)]
            for j in range(0, n):
                for h in range(9 + 1):
                    if j == 0:
                        mask[j][h] = 1
                    if grid[i][j] > h:
                        mask[j][h] = 0
                    else:
                        mask[j][h] = mask[j - 1][h] + 1
                if j != 0:
                    prev = mask[j - 1][grid[i][j] - 1] if grid[i][j] != 0 else 0
                    ans[i][j] *= (
                        0
                        + prev
                        + (prev != mask[j - 1][9])
                    )
                else:
                    ans[i][j] = 0
        grid = rot(grid)
        ans = rot(ans)

    show(ans)
    print(max(max(row) for row in ans))

if __name__ == "__main__":
    main()
