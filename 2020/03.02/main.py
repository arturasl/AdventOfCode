import sys


def main():
    map = []
    for line in sys.stdin:
        map.append(line.strip())

    result = 1
    for dy, dx in [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]:
        pos = (0, 0)
        num_trees = 0
        while True:
            pos = (pos[0] + dy, pos[1] + dx)
            if pos[0] >= len(map):
                break
            line = map[pos[0]]
            ch = line[pos[1] % len(line)]
            num_trees += ch == "#"
        result *= num_trees

    print(result)


if __name__ == "__main__":
    main()
