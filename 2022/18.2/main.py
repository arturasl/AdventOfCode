import sys
from collections import defaultdict

def calc_surface_area(cubes):
    exposed = defaultdict(int)
    for c in cubes:
        for i in range(len(c)):
            for off in [-1, +1]:
                new_coord = list(c)
                new_coord[i] += off
                exposed[tuple(new_coord)] +=1

    over = sum(v for k, v in exposed.items() if k in cubes)
    return len(cubes) * 6 - over

def visit(ranges, cubes, start):
    visited = set([])
    queue = [start]
    while queue:
        pos = queue.pop()
        if pos in visited:
            continue
        if pos in cubes:
            continue
        visited.add(pos)
        for i in range(len(pos)):
            for off in [-1, +1]:
                new_pos = list(pos)
                new_pos[i] += off
                if any(not (ranges[i][0] <= new_pos[i] <= ranges[i][1]) for i in range(len(ranges))):
                    continue
                queue.append(tuple(new_pos))
    return visited

def find_gaps(ranges, cubes):
    gaps = set([])
    for i in range(ranges[0][0], ranges[0][1] + 1):
        for j in range(ranges[1][0], ranges[1][1] + 1):
            for k in range(ranges[2][0], ranges[2][1] + 1):
                if (i, j, k) not in cubes:
                    gaps.add((i, j, k))
    return gaps

def main():
    cubes = set([tuple(int(x) for x in line.strip().split(',')) for line in sys.stdin if line])
    ranges = [tuple(f(c[i] + o for c in cubes) for f, o in [(min, -1), (max, +1)]) for i in range(3)]
    visited = visit(ranges, cubes, tuple(r[0] for r in ranges))
    gaps = find_gaps(ranges, cubes | visited)
    print(calc_surface_area(cubes | gaps))

if __name__ == "__main__":
    main()
