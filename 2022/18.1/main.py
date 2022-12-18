import sys
from collections import defaultdict

def main():
    exposed = defaultdict(int)
    cubes = set([tuple(int(x) for x in line.strip().split(',')) for line in sys.stdin if line])
    for c in cubes:
        for i in range(len(c)):
            for off in [-1, +1]:
                new_coord = list(c)
                new_coord[i] += off
                exposed[tuple(new_coord)] +=1

    over = 0
    for k, v in exposed.items():
        if k in cubes:
            over += v

    print(len(cubes) * 6 - over)

if __name__ == "__main__":
    main()
