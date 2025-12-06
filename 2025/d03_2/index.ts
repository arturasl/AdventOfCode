import readline from "node:readline";
import assert from "node:assert";

function maxi(arr: string, start: number, end: number) {
  assert(start < end);
  let best_idx = start;
  for (let idx = start + 1; idx < end; idx += 1) {
    if (arr[best_idx]! < arr[idx]!) {
      best_idx = idx;
    }
  }
  return best_idx;
}

async function main() {
  const find_of_len = 12;
  let sum = 0n;
  for await (const line of readline.createInterface({
    input: process.stdin,
  })) {
    let result = "";
    let start = 0;
    assert(line.length >= find_of_len);
    for (let i = find_of_len; i > 0; i -= 1) {
      const pos = maxi(line, start, line.length - i + 1);
      start = pos + 1;
      result += line[pos];
    }

    sum += BigInt(result);
  }
  console.log(sum.toString());
}

main();
