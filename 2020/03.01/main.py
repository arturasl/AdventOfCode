import sys


def main():
    map = []
    for line in sys.stdin:
        map.append(line.strip())

    pos = (0, 0)
    num_trees = 0
    while True:
        pos = (pos[0] + 1, pos[1] + 3)
        if pos[0] >= len(map):
            break
        line = map[pos[0]]
        ch = line[pos[1] % len(line)]
        num_trees += ch == "#"

    print(num_trees)


if __name__ == "__main__":
    main()
