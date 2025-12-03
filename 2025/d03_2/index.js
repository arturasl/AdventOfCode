import readline from "node:readline";

function maxi(arr) {
  const pos = arr.reduce(
    (best_idx, cur_val, cur_idx) =>
      best_idx == null || arr[best_idx] < cur_val ? cur_idx : best_idx,
    null,
  );
  return { pos, val: pos == null ? "" : arr[pos] };
}

async function main() {
  let sum = 0n;
  for await (const line of readline.createInterface({
    input: process.stdin,
  })) {
    const arr = Array.from(line);
    let result = [];
    let start = 0;
    for (let i = 12; i > 0; i -= 1) {
      const { pos, val } = maxi(arr.slice(start, arr.length - i + 1));
      start += pos + 1;
      result.push(val);
    }

    sum += BigInt(result.join(""));
  }
  console.log(sum);
}

main();
