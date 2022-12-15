import sys
from collections import deque

def main():
    grid = [list(line.strip()) for line in sys.stdin if line]
    visited = []

    d = deque()

    for i in range(len(grid)):
        visited.append([False for _ in range(len(grid[i]))])
        for j in range(len(grid[i])):
          if grid[i][j] == 'a':
              d.append(((i, j), 0))

    def get_height(i, j):
        ch = grid[i][j]
        if ch == 'S':
            ch = 'a'
        if ch == 'E':
            ch = 'z'
        return ord(ch) - ord('a')

    while d:
        pos, dist = d.popleft()
        if grid[pos[0]][pos[1]] == 'E':
            print(dist)
            break

        for move in [(-1, 0), (+1, 0), (0, -1), (0, +1)]:
            next_pos = (pos[0] + move[0], pos[1] + move[1])
            if not (0 <= next_pos[0] < len(grid)):
                continue
            if not (0 <= next_pos[1] < len(grid[next_pos[0]])):
                continue
            if visited[next_pos[0]][next_pos[1]]:
                continue
            if get_height(next_pos[0], next_pos[1]) > get_height(pos[0], pos[1]) + 1:
                continue
            visited[next_pos[0]][next_pos[1]] = True
            d.append((next_pos, dist + 1))

if __name__ == "__main__":
    main()
