import sys
from collections import defaultdict


def main():
    num_to_cnt: dict[int, int] = defaultdict(int)
    for line in sys.stdin:
        line = line.strip()
        num_to_cnt[int(line)] += 1

    result: set[int] = set([])
    for k in num_to_cnt.keys():
        search_for = 2020 - k
        min_cnt = 2 if search_for == k else 1
        if num_to_cnt.get(search_for, 0) >= min_cnt:
            result.add(k * search_for)

    assert len(result) == 1
    print(result.pop())


if __name__ == "__main__":
    main()
