import sys
from collections import defaultdict


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
    diffs: dict[int, int] = defaultdict(int)
    for i in range(1, len(nums)):
        diffs[nums[i] - nums[i - 1]] += 1

    assert 0 <= max(diffs.keys()) <= 3
    print(diffs[1] * diffs[3])


if __name__ == "__main__":
    main()
