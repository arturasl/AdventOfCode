import sys
from collections.abc import Hashable

from cachetools import cached
from cachetools.keys import hashkey


def count_cache(_: list[int], idx: int, prev_val: int) -> tuple[Hashable, ...]:
    return hashkey((idx, prev_val))


@cached(cache={}, key=count_cache)
def count(nums: list[int], idx: int, prev_val: int):
    if len(nums) == idx:
        return 1

    ans = 0
    while idx < len(nums) and nums[idx] - prev_val <= 3:
        ans += count(nums, idx + 1, nums[idx])
        idx += 1

    return ans


def main():
    nums: list[int] = []
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        nums.append(int(line))

    nums.append(0)
    nums.append(max(nums) + 3)
    nums = sorted(nums)
    print(count(nums, 1, nums[0]))


if __name__ == "__main__":
    main()
