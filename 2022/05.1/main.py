import sys
import re

def read_stacks():
    stacks = []
    for line in sys.stdin:
        if not line.strip():
            break
        parts = [line[i:i+4].strip().strip('[]') for i in range(0, len(line), 4)]
        if parts[0] == '1':
            continue
        while len(stacks) < len(parts):
            stacks.append([])
        for i, val in enumerate(parts):
            if val:
                stacks[i].append(val)
    stacks = [stack[::-1] for stack in stacks]
    return stacks

def move(stacks):
    for line in sys.stdin:
        g = re.match(r'^move (\d+) from (\d+) to (\d+)$', line.strip())
        c, f, t = [int(g.group(i + 1)) for i in range(3)]
        for _ in range(c):
            stacks[t - 1].append(stacks[f - 1].pop())
    return stacks

def main():
    stacks = read_stacks()
    stacks = move(stacks)
    print(''.join([stack[-1] for stack in stacks]))

if __name__ == "__main__":
    main()
