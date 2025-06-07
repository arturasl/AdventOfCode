import sys


def solve(inp: int, moves: int) -> int:
    cups: list[int] = []
    while inp:
        cups.append(inp % 10)
        inp //= 10
    cups = cups[::-1]

    cur_idx = 0

    for _ in range(moves):
        print("#########")
        print(
            f"cups: {[c if i != cur_idx else '(' + str(c) + ')' for i, c in enumerate(cups)]}"
        )
        removed_labels: list[int] = []
        for _ in range(3):
            removed_idx = (cur_idx + 1) % len(cups)
            if removed_idx < cur_idx:
                cur_idx -= 1
            removed_labels.append(cups[removed_idx])
            del cups[removed_idx]
        print(f"pick up: {removed_labels}")

        dest_label = (cups[cur_idx] - 1) % 10
        while dest_label not in cups:
            dest_label = (dest_label - 1) % 10
        print(f"destination: {dest_label}")
        dest_idx = cups.index(dest_label)
        for removed_label in reversed(removed_labels):
            if dest_idx < cur_idx:
                cur_idx += 1
            cups.insert(dest_idx + 1, removed_label)
        cur_idx = (cur_idx + 1) % len(cups)

    one_idx = cups.index(1)
    return int("".join(str(cup) for cup in cups[one_idx + 1 :] + cups[:one_idx]))


def test_first():
    assert solve(389125467, 10) == 92658374


def test_second():
    assert solve(389125467, 100) == 67384529


def main():
    inp, moves = sys.stdin.readline().strip().split(" ")
    print(solve(int(inp), int(moves)))


if __name__ == "__main__":
    main()
