import sys
from collections import defaultdict

def main():
    cycle = 1
    x = 1
    r = 0
    instructions = []
    for parts in (line.strip().split() for line in sys.stdin if line):
        if parts[0] == 'addx':
            instructions.extend([['noop']] * 2)
        instructions.append(parts)

    for parts in instructions:
        if parts[0] == 'noop':
            if (cycle - 20) % 40 == 0:
                r += cycle * x
            cycle += 1
        elif parts[0] == 'addx':
            x += int(parts[1])
        else:
            assert False, parts

    print(r)

if __name__ == "__main__":
    main()
