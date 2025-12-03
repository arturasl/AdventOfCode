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
  let sum = 0;
  for await (const line of readline.createInterface({
    input: process.stdin,
  })) {
    const arr = Array.from(line);
    const { pos, val: mid_val } = maxi(arr);
    const { val: lhs_val } = maxi(arr.slice(0, pos));
    const { val: rhs_val } = maxi(arr.slice(pos + 1));

    sum += parseInt((lhs_val + mid_val + rhs_val).slice(-2), 10);
  }
  console.log(sum);
}

main();
