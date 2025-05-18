from __future__ import annotations

import re
import sys
from collections import defaultdict
from dataclasses import dataclass
from enum import Enum


class Bit(Enum):
    ZERO = 0
    ONE = 1
    EITHER = 2


@dataclass
class SetMemAction:
    addr_bits: list[Bit]
    val: int


def calc(actions: list[SetMemAction], pos: int = 0) -> int:
    if not actions:
        return 0
    if len(actions[0].addr_bits) == pos:
        return actions[-1].val

    if all(action.addr_bits[pos] == Bit.EITHER for action in actions):
        return 2 * calc(actions, pos + 1)

    split: dict[Bit, list[SetMemAction]] = defaultdict(list)
    for bit in [Bit.ZERO, Bit.ONE]:
        for action in actions:
            if action.addr_bits[pos] in [Bit.EITHER, bit]:
                split[bit].append(action)

    ans = 0
    for vals in split.values():
        ans += calc(vals, pos + 1)

    return ans


def lines_to_actions(lines: list[str], num_bits: int = 36) -> list[SetMemAction]:
    actions: list[SetMemAction] = []
    mask: list[str] = ["X" for _ in range(num_bits)]
    re_mem = re.compile(r"^mem\[(\d+)\]$")
    for line in lines:
        parts = line.split(" = ")
        assert len(parts) == 2

        if parts[0] == "mask":
            assert len(parts[1]) == num_bits
            mask = list(parts[1])
        else:
            grp = re_mem.match(parts[0])
            assert grp
            addr = list(bin(int(grp[1]))[2:])
            addr = ["0" for _ in range(num_bits - len(addr))] + addr
            addr = [a if m == "0" else m for m, a in zip(mask, addr)]
            addr = [{"0": Bit.ZERO, "1": Bit.ONE, "X": Bit.EITHER}[a] for a in addr]
            actions.append(SetMemAction(addr, int(parts[1])))
    return actions


def solve(lines: list[str], num_bits: int = 36):
    actions = lines_to_actions(lines, num_bits)
    return calc(actions)


def main():
    lines = [line for line in [line.strip() for line in sys.stdin] if line]
    print(solve(lines))


if __name__ == "__main__":
    main()


def test_solve_ex():
    assert (
        solve(
            [
                "mask = 000000000000000000000000000000X1001X",
                "mem[42] = 100",
                "mask = 00000000000000000000000000000000X0XX",
                "mem[26] = 1",
            ]
        )
        == 208
    )


def test_solve_1():
    assert (
        solve(
            [
                "mask = 00000",
                "mem[0] = 123",
            ],
            5,
        )
        == 123
    )


def test_solve_2():
    assert (
        solve(
            [
                "mask = 0000X",
                "mem[0] = 123",
            ],
            5,
        )
        == 246
    )


def test_solve_3():
    assert (
        solve(
            [
                "mask = 000XX",
                "mem[0] = 123",
            ],
            5,
        )
        == 492
    )


def test_solve_4():
    assert (
        solve(
            [
                "mask = 000XX",
                "mem[0] = 123",
                "mem[0] = 1",
            ],
            5,
        )
        == 4
    )


def test_solve_5():
    assert (
        solve(
            [
                "mask = 000XX",
                "mem[0] = 123",
                "mask = 0000X",
                "mem[0] = 1",
            ],
            5,
        )
        == 248
    )


def test_solve_6():
    assert (
        solve(
            [
                "mask = 000XX",
                "mem[0] = 123",
                "mask = 000X0",
                "mem[0] = 1",
            ],
            5,
        )
        == 248
    )
