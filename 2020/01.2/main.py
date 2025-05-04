import sys
from collections import defaultdict


def main():
    num_to_cnt = defaultdict(int)
    nums = []
    for line in sys.stdin:
        line = line.strip()
        num = int(line)
        num_to_cnt[num] += 1
        nums.append(num)

    result = set([])
    for i in range(0, len(nums)):
        for j in range(i + 1, len(nums)):
            num_to_cnt[nums[i]] -= 1
            num_to_cnt[nums[j]] -= 1
            search_for = 2020 - nums[i] - nums[j]
            if num_to_cnt.get(search_for, 0) > 0:
                result.add(nums[i] * nums[j] * search_for)
            num_to_cnt[nums[i]] += 1
            num_to_cnt[nums[j]] += 1

    assert len(result) == 1
    print(result.pop())


if __name__ == "__main__":
    main()
