import sys
from collections import defaultdict


def solve(numbers: list[int], cnt: int) -> int:
    spoken: dict[int, list[int]] = defaultdict(list)
    idx = 0
    prev = -1
    while idx < len(numbers):
        spoken[numbers[idx]].append(idx)
        prev = numbers[idx]
        idx += 1

    while idx < cnt:
        if len(spoken[prev]) <= 1:
            prev = 0
        else:
            prev = idx - spoken[prev][-2] - 1
        spoken[prev].append(idx)
        idx += 1

    assert prev != -1
    return prev


def main():
    print(solve([int(x) for x in sys.stdin.readline().strip().split(",")], 2020))


def test_1():
    assert solve([1, 3, 2], 2020) == 1


def test_2():
    assert solve([2, 1, 3], 2020) == 10


def test_3():
    assert solve([1, 2, 3], 2020) == 27


def test_4():
    assert solve([2, 3, 1], 2020) == 78


def test_5():
    assert solve([3, 2, 1], 2020) == 438


def test_6():
    assert solve([3, 1, 2], 2020) == 1836


if __name__ == "__main__":
    main()
