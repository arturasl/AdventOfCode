import readline from "node:readline";
import assert from "node:assert";

function maxi(arr: string[]): { pos: number | null; val: string } {
  const pos: number | null = arr.reduce(
    (best_idx: number | null, cur_val: string, cur_idx: number): number =>
      best_idx === null || arr[best_idx]! < cur_val ? cur_idx : best_idx,
    null,
  );
  return { pos, val: pos === null ? "" : arr[pos]! };
}

async function main() {
  let sum = 0;
  for await (const line of readline.createInterface({
    input: process.stdin,
  })) {
    const arr = Array.from(line);
    const { pos, val: mid_val } = maxi(arr);
    assert(pos != null);
    const { val: lhs_val } = maxi(arr.slice(0, pos!));
    const { val: rhs_val } = maxi(arr.slice(pos! + 1));

    sum += +(lhs_val + mid_val + rhs_val).slice(-2);
  }
  console.log(sum);
}

main();
