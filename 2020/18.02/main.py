from __future__ import annotations

import sys
from dataclasses import dataclass
from typing import override


@dataclass
class Op:
    def execute(self) -> int:
        assert False


@dataclass
class Num(Op):
    n: int

    @override
    def execute(self: Num) -> int:
        return self.n


@dataclass
class Mul(Op):
    lhs: Op
    rhs: Op

    @override
    def execute(self: Mul) -> int:
        return self.lhs.execute() * self.rhs.execute()


@dataclass
class Add(Op):
    lhs: Op
    rhs: Op

    @override
    def execute(self: Add) -> int:
        return self.lhs.execute() + self.rhs.execute()


def create_op(
    sign: str, operands: list[Op], ops: list[str]
) -> tuple[list[Op], list[str]]:
    erase_idx: set[int] = set()
    for idx, op in enumerate(ops):
        if op != sign:
            continue
        erase_idx.add(idx)
        operands[idx + 1] = {"+": Add, "*": Mul}[ops[idx]](
            operands[idx], operands[idx + 1]
        )

    ops = [op for i, op in enumerate(ops) if i not in erase_idx]
    operands = [op for i, op in enumerate(operands) if i not in erase_idx]

    return (operands, ops)


def parse(txt: str, i: int) -> tuple[int, Op]:
    assert txt[i] == "("
    i += 1

    operands: list[Op] = []
    ops: list[str] = []

    while True:
        lhs = None
        if txt[i].isdigit():
            num = 0
            while i != len(txt) and txt[i].isdigit():
                num = num * 10 + int(txt[i])
                i += 1
            lhs = Num(num)
        else:
            i, lhs = parse(txt, i)

        operands.append(lhs)

        if txt[i] == ")":
            break

        assert txt[i] in "+*"
        ops.append(txt[i])
        i += 1
    i += 1

    operands, ops = create_op("+", operands, ops)
    operands, ops = create_op("*", operands, ops)
    assert not ops
    assert len(operands) == 1

    return (i, operands[0])


def solve(txt: str) -> int:
    txt = "(" + txt.strip().replace(" ", "") + ")"
    i, op = parse(txt, 0)
    assert i == len(txt)
    return op.execute()


def main():
    print(sum(solve(line) for line in sys.stdin if line.strip()))


def test_inp_m1():
    assert solve("2 * 3") == 6


def test_inp_m2():
    assert solve("10 * 123") == 1230


def test_inp_1():
    assert solve("1 + (2 * 3) + (4 * (5 + 6))") == 51


def test_inp_2():
    assert solve("2 * 3 + (4 * 5)") == 46


def test_inp_3():
    assert solve("5 + (8 * 3 + 9 + 3 * 4 * 3)") == 1445


def test_inp_4():
    assert solve("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))") == 669060


def test_inp_5():
    assert solve("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2") == 23340


if __name__ == "__main__":
    main()
