import sys
from dataclasses import dataclass
from enum import Enum


class Op(Enum):
    NOP = 0
    ACC = 1
    JMP = 2

    @staticmethod
    def from_str(s: str):
        return {"nop": Op.NOP, "acc": Op.ACC, "jmp": Op.JMP}[s]


@dataclass
class Instruction:
    op: Op
    amount: int


def main():
    instructions: list[Instruction] = []
    for line in sys.stdin:
        line = line.strip()
        op, amount = line.split()
        instructions.append(Instruction(Op.from_str(op), int(amount)))

    idx = 0
    accumualtor = 0
    executed_idxes: set[int] = set()
    while True:
        if idx in executed_idxes:
            print(accumualtor)
            break
        executed_idxes.add(idx)
        instruction = instructions[idx]

        match instruction.op:
            case Op.ACC:
                accumualtor += instruction.amount
                idx += 1
            case Op.NOP:
                idx += 1
            case Op.JMP:
                idx += instruction.amount


if __name__ == "__main__":
    main()
