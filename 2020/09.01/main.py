import sys
from collections import defaultdict, deque

PREMABLE_SIZE = 25


def main():
    previous = deque()
    previous_mp = defaultdict(int)
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        num = int(line)

        if len(previous) == PREMABLE_SIZE:
            if not any(
                (num - prev_num) in previous_mp and (num - prev_num) != prev_num
                for prev_num in previous
            ):
                print(num)
                break

            dropped_num = previous.popleft()
            previous_mp[dropped_num] -= 1
            if previous_mp[dropped_num] == 0:
                del previous_mp[dropped_num]
        previous.append(num)
        previous_mp[num] += 1


if __name__ == "__main__":
    main()
