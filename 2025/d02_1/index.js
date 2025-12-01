import readline from "node:readline";
import assert from "node:assert";

async function main() {
  let ranges = [];
  for await (const line of readline.createInterface({
    input: process.stdin,
  })) {
    for (const range of line.split(",")) {
      const [str_lhs, str_rhs] = range.split("-");
      ranges.push({ lhs: BigInt(str_lhs), rhs: BigInt(str_rhs) });
    }
  }

  const upper_bound = ranges
    .map(({ lhs }) => lhs)
    .reduce((prev, cur) => (prev > cur ? prev : cur));

  let valid = [];
  for (let base = 1n; base * base * 10n + base <= upper_bound; base *= 10n) {
    for (let num = base; num < base * 10n; num += 1n) {
      valid.push(num * base * 10n + num);
    }
  }

  for (let i = 0; i < valid.length - 1; i += 1) {
    assert(valid[i] < valid[i + 1]);
  }

  let prefix_sum = [0n];
  for (const el of valid) {
    prefix_sum.push(prefix_sum.at(-1) + el);
  }

  // Smallest valid >= n.
  const find_pos = (n) => {
    let low = 0;
    let high = valid.length - 1;
    while (low <= high) {
      let mid = low + ((high - low) >> 1);

      if (valid[mid] < n) {
        low = mid + 1;
      } else {
        high = mid - 1;
      }
    }
    return low;
  };

  let sum = 0n;
  for (const { lhs, rhs } of ranges) {
    sum += prefix_sum[find_pos(rhs + 1n)] - prefix_sum[find_pos(lhs)];
  }
  console.log(sum);
}

main();
