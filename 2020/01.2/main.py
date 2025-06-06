import itertools
import sys
from collections import defaultdict


def main():
    num_to_cnt: dict[int, int] = defaultdict(int)
    nums: list[int] = []
    for line in sys.stdin:
        line = line.strip()
        num = int(line)
        num_to_cnt[num] += 1
        nums.append(num)

    result: set[int] = set([])
    for lhs, rhs in itertools.combinations(nums, 2):
        num_to_cnt[lhs] -= 1
        num_to_cnt[rhs] -= 1
        search_for = 2020 - lhs - rhs
        if num_to_cnt.get(search_for, 0) > 0:
            result.add(lhs * rhs * search_for)
        num_to_cnt[lhs] += 1
        num_to_cnt[rhs] += 1

    assert len(result) == 1
    print(result.pop())


if __name__ == "__main__":
    main()
