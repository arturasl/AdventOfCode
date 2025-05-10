import sys
from dataclasses import dataclass
from enum import Enum


class Op(Enum):
    NOP = 0
    ACC = 1
    JMP = 2

    @staticmethod
    def from_str(s):
        return {"nop": Op.NOP, "acc": Op.ACC, "jmp": Op.JMP}[s]


@dataclass
class Instruction:
    op: Op
    amount: int


def execute(instructions):
    idx = 0
    accumualtor = 0
    executed_idxes = set()
    while True:
        if idx in executed_idxes:
            return None
        if idx == len(instructions):
            return accumualtor

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


def main():
    instructions = []
    for line in sys.stdin:
        line = line.strip()
        op, amount = line.split()
        instructions.append(Instruction(Op.from_str(op), int(amount)))

    for idx in range(len(instructions)):
        old = instructions[idx]

        instructions[idx] = Instruction(
            {Op.NOP: Op.JMP, Op.ACC: Op.ACC, Op.JMP: Op.NOP}[old.op], old.amount
        )
        result = execute(instructions)
        if result:
            print(result)

        instructions[idx] = old


if __name__ == "__main__":
    main()
