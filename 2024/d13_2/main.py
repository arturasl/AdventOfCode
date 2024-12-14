import sys
import re
from z3 import *

OFT = 10000000000000

def read_none_empty():
    prev = sys.stdin.tell()
    while True:
        line = sys.stdin.readline().strip()
        cur = sys.stdin.tell()

        if not line:
            if prev == cur:
                return None
            continue

        prev = cur
        return line

def read_button():
    ln = read_none_empty()
    if not ln:
        return None, None

    m = re.match(r"^Button [AB]: X(?P<x>[+-]\d+), Y(?P<y>[+-]\d+)$", ln)
    return int(m.group('x')), int(m.group('y'))

def read_prize():
    ln = read_none_empty()
    if not ln:
        return None, None

    m = re.match(r"Prize: X=(?P<x>\d+), Y=(?P<y>\d+)$", ln)
    return int(m.group('x')) + OFT, int(m.group('y')) + OFT

def main():
    result = 0

    while True:
        a_x, a_y = read_button()
        b_x, b_y = read_button()
        p_x, p_y = read_prize()

        if a_x is None:
            break

        a_p, b_p = Int('ap'), Int('bp')

        s = Optimize()

        s.add(a_x * a_p + b_x * b_p == p_x)
        s.add(a_y * a_p + b_y * b_p == p_y)
        s.minimize(a_p * 3 + b_p)

        s.check()
        m = s.model()
        if m:
            result += m[a_p].as_long() * 3 + m[b_p].as_long()

    print(result)

if __name__ == "__main__":
    main()
