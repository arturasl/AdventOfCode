import sys
from functools import reduce
from operator import and_

def main():
    points = 0
    lines = [line.strip() for line in sys.stdin if line]
    for grp in (lines[i:i+3] for i in range(0, len(lines), 3)):
        c = list(reduce(and_, [set(g) for g in grp]))[0]
        points += ord(c) - (ord('A') - 26 if 'A' <= c <= 'Z' else ord('a')) + 1
    print(f'{points}')

if __name__ == "__main__":
    main()
