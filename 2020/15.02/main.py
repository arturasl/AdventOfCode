import sys


def solve(numbers: list[int], cnt: int) -> int:
    spoken: dict[int, int] = {}
    idx = 0
    while idx < len(numbers) - 1:
        spoken[numbers[idx]] = idx
        idx += 1

    next_num = numbers[idx]

    while idx < cnt - 1:
        cur_num = idx - spoken.get(next_num, idx)
        spoken[next_num] = idx
        next_num = cur_num
        idx += 1

    return next_num


def main():
    print(solve([int(x) for x in sys.stdin.readline().strip().split(",")], 2020))


def test_0():
    assert solve([0, 3, 5], 10) == 0


def test_1():
    assert solve([1, 3, 2], 2020) == 1


def test_1_big():
    assert solve([1, 3, 2], 30000000) == 2578


def test_2():
    assert solve([2, 1, 3], 2020) == 10


def test_2_big():
    assert solve([2, 1, 3], 30000000) == 3544142


def test_3():
    assert solve([1, 2, 3], 2020) == 27


def test_3_big():
    assert solve([1, 2, 3], 30000000) == 261214


def test_4():
    assert solve([2, 3, 1], 2020) == 78


def test_4_big():
    assert solve([2, 3, 1], 30000000) == 6895259


def test_5():
    assert solve([3, 2, 1], 2020) == 438


def test_5_big():
    assert solve([3, 2, 1], 30000000) == 18


def test_6():
    assert solve([3, 1, 2], 2020) == 1836


def test_6_big():
    assert solve([3, 1, 2], 30000000) == 362


def test_7_big():
    assert solve([1, 0, 16, 5, 17, 4], 30000000) == 362


if __name__ == "__main__":
    main()
