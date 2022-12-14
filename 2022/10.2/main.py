import sys

def main():
    cycle = 1
    x = 1
    instructions = []
    for parts in (line.strip().split() for line in sys.stdin if line):
        if parts[0] == 'addx':
            instructions.extend([['noop']] * 2)
        instructions.append(parts)

    drawing = []
    for parts in instructions:
        if parts[0] == 'noop':
            drawing.append('#' if abs((cycle - 1) % 40 - x) <= 1 else '.')
            if cycle % 40 == 0:
                drawing.append('\n')
            cycle += 1
        elif parts[0] == 'addx':
            x += int(parts[1])
        else:
            assert False, parts

    print(''.join(drawing))

if __name__ == "__main__":
    main()
