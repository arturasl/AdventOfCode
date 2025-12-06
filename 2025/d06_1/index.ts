import readline from "node:readline";
import assert from "node:assert";

async function main() {
  const Op = {
    mul: "*",
    add: "+",
  } as const;
  type Op = (typeof Op)[keyof typeof Op];

  const matrix: number[][] = [];
  const ops: Op[] = [];

  for await (let line of readline.createInterface({
    input: process.stdin,
  })) {
    line = line.trim();
    if (!line) {
      continue;
    }

    const nums: number[] = Array.from(
      line.matchAll(/\d+/g),
      (entry) => +entry[0],
    );
    if (nums.length) {
      assert(!matrix.length || matrix.at(-1)!.length == nums.length);
      matrix.push(nums);
      continue;
    }

    assert(ops.length == 0);
    ops.push(
      ...Array.from(line.matchAll(/[+*]/g), (entry) => {
        return entry[0] as Op;
      }),
    );
  }

  assert(matrix.length && ops.length == matrix.at(-1)!.length);

  const op_funcs: Record<Op, (a: number, b: number) => number> = {
    [Op.mul]: (a: number, b: number) => a * b,
    [Op.add]: (a: number, b: number) => a + b,
  };
  let col_sols: number[] = [];
  for (let col_idx = 0; col_idx < ops.length; col_idx++) {
    let sol = ops[col_idx] == "*" ? 1 : 0;
    const op_func = op_funcs[ops[col_idx]!];
    for (let row_idx = 0; row_idx < matrix.length; row_idx++) {
      sol = op_func(sol, matrix[row_idx]![col_idx]!);
    }
    col_sols.push(sol);
  }

  console.log(col_sols.reduce(op_funcs["+"]));
}

main();
