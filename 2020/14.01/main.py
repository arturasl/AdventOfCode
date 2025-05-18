import re
import sys
from collections import defaultdict

NUM_BITS = 36


def main():
    mask: list[int | None] = [None for _ in range(NUM_BITS)]
    mem: dict[int, list[int]] = defaultdict(lambda: [0 for _ in range(NUM_BITS)])
    re_mem = re.compile(r"^mem\[(\d+)\]$")

    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        parts = line.split(" = ")
        assert len(parts) == 2

        if parts[0] == "mask":
            assert len(parts[1]) == NUM_BITS
            for i in range(NUM_BITS):
                match parts[1][i]:
                    case "X":
                        mask[i] = None
                    case "1":
                        mask[i] = 1
                    case "0":
                        mask[i] = 0
                    case _:
                        assert False, parts[i]
        else:
            grp = re_mem.match(parts[0])
            assert grp
            addr = int(grp[1])
            val = [int(x) for x in list(bin(int(parts[1]))[2:])]
            val = [0 for _ in range(NUM_BITS - len(val))] + val
            val = [v if m is None else m for m, v in zip(mask, val)]
            mem[addr] = val

    ans = sum(int("".join(str(v) for v in val), 2) for val in mem.values())
    print(ans)


if __name__ == "__main__":
    main()
