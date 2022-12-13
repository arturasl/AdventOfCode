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
        print(''.join(str(int(r)) for r in row))
    print('')

def main():
    grid = [[int(ch) for ch in line.strip()] for line in sys.stdin]
    assert len(grid) == len(grid[0])
    n = len(grid)
    visible = [[False for _ in range(n)] for _ in range(n)]

    for _ in range(4):
        show(grid)

        for i in range(n):
            prev = -1
            for j in range(n):
                if prev < grid[i][j]:
                    visible[i][j] = True
                    prev = grid[i][j]
        grid = rot(grid)
        visible = rot(visible)

    show(grid)
    show(visible)
    print(sum(sum(row) for row in visible))

if __name__ == "__main__":
    main()
