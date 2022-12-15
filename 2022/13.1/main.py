import sys
from collections import deque
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
    idx = 1
    r = 0
    it = iter(sys.stdin)
    try:
        while True:
          line1 = json.loads(next(it).strip())
          line2 = json.loads(next(it).strip())

          c = cmp(line1, line2)
          assert c != 0
          if c == -1:
              r += idx

          assert next(it).strip() == ''
          idx += 1
    except StopIteration:
        pass

    print(r)

if __name__ == "__main__":
    main()
