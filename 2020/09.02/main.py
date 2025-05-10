import sys
from collections import defaultdict, deque

PREMABLE_SIZE = 25


def fidn_wrong_num(nums):
    previous = deque()
    previous_mp = defaultdict(int)
    for num in nums:
        if len(previous) == PREMABLE_SIZE:
            if not any(
                (num - prev_num) in previous_mp and (num - prev_num) != prev_num
                for prev_num in previous
            ):
                return num

            dropped_num = previous.popleft()
            previous_mp[dropped_num] -= 1
            if previous_mp[dropped_num] == 0:
                del previous_mp[dropped_num]
        previous.append(num)
        previous_mp[num] += 1


def sum_between_inc(a, b, acc):
    return acc[b + 1] - acc[a]


def main():
    nums = []
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        nums.append(int(line))

    wrong = fidn_wrong_num(nums)

    acc = [0]
    prev_to_acc_pos = {0: 0}
    for idx, num in enumerate(nums):
        s = acc[-1] + num

        search = s - wrong
        if s - wrong in prev_to_acc_pos:
            prev_idx = prev_to_acc_pos[search]
            if idx != prev_idx:
                print(max(nums[prev_idx : idx + 1]) + min(nums[prev_idx : idx + 1]))

        acc.append(s)
        prev_to_acc_pos[s] = idx + 1


if __name__ == "__main__":
    main()
