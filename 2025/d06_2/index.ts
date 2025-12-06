import readline from "node:readline";
import assert from "node:assert";

async function main() {
  type Op = "*" | "+";

  const ops: Op[] = [];
  const blocks: string[] = [];

  for await (let line of readline.createInterface({
    input: process.stdin,
  })) {
    line = line.trimEnd();
    if (!line) {
      continue;
    }

    const maybe_ops = Array.from(line.matchAll(/[+*]/g), (entry) => {
      return entry[0] as Op;
    });
    if (maybe_ops.length) {
      ops.push(...maybe_ops);
      continue;
    }

    blocks.push(line);
  }

  let matrix: number[][] = [[]];
  let max_len = blocks
    .map((line) => line.length)
    .reduce((a, b) => Math.max(a, b));
  for (let x = 0; x < max_len; x += 1) {
    let col = "";
    for (let y = 0; y < blocks.length; y += 1) {
      col += blocks?.[y]?.[x] ?? " ";
    }
    col = col.trim();
    if (col) {
      matrix.at(-1)!.push(+col);
    } else {
      matrix.push([]);
    }
  }

  assert(matrix.length == ops.length);

  const op_funcs: Record<Op, (a: number, b: number) => number> = {
    "*": (a: number, b: number) => a * b,
    "+": (a: number, b: number) => a + b,
  };

  let col_sols: number[] = [];
  for (let i = 0; i < ops.length; i++) {
    let sol = ops[i] == "*" ? 1 : 0;
    const op_func = op_funcs[ops[i]!];
    for (const num of matrix[i]!) {
      sol = op_func(sol, num);
    }
    col_sols.push(sol);
  }

  console.log(col_sols.reduce(op_funcs["+"]));
}

main();
