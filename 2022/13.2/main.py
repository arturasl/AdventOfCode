import sys
import functools
import json

def sign(x):
    if x < 0:
        return -1
    if x > 0:
        return +1
    return 0

def cmp(lhs, rhs):
    for lhs_item, rhs_item in zip(lhs, rhs):
        if type(lhs_item) != type(rhs_item):
            if type(lhs_item) == type(0):
                lhs_item = [lhs_item]
            else:
                rhs_item = [rhs_item]

        result = None
        if type(lhs_item) == type(0):
            result = sign(lhs_item - rhs_item)
        else:
            result = cmp(lhs_item, rhs_item)

        if result != 0:
            return result

    return sign(len(lhs) - len(rhs))

def main():
    lines = [json.loads(line.strip()) for line in sys.stdin if line.strip()]
    lines.extend([[[2]], [[6]]])
    lines = sorted(lines, key=functools.cmp_to_key(cmp))
    lines = [str(line) for line in lines]

    print((lines.index('[[2]]') + 1) * (lines.index('[[6]]') + 1))

if __name__ == "__main__":
    main()
