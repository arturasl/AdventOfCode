import readline from "node:readline";

function repeat(num: bigint, base: bigint, times: bigint): bigint {
  let result = 0n;
  for (let i = 0; i < times; i += 1) {
    result = result * base * 10n + num;
  }
  return result;
}

async function main() {
  let ranges = [];
  for await (let line of readline.createInterface({
    input: process.stdin,
  })) {
    line = line.replace(/,+$/, "");
    for (const range of line.split(",")) {
      const [str_lhs, str_rhs] = range.split("-");
      ranges.push({ lhs: BigInt(str_lhs!), rhs: BigInt(str_rhs!) });
    }
  }

  const upper_bound = ranges
    .map(({ lhs }) => lhs)
    .reduce((prev, cur) => (prev > cur ? prev : cur));

  let valid = [];
  for (let base = 1n; repeat(base, base, 2n) <= upper_bound; base *= 10n) {
    for (
      let times = 2n;
      repeat(base, base, times) <= upper_bound;
      times += 1n
    ) {
      for (let num = base; num < base * 10n; num += 1n) {
        valid.push(repeat(num, base, times));
      }
    }
  }
  valid = Array.from(new Set(valid)).sort((a, b) =>
    a > b ? 1 : a == b ? 0 : -1,
  );

  let prefix_sum = [0n];
  for (const el of valid) {
    prefix_sum.push(prefix_sum.at(-1)! + el);
  }

  // Smallest valid >= n.
  const find_pos = (n: bigint): number => {
    let low = 0;
    let high = valid.length - 1;
    while (low <= high) {
      let mid = low + ((high - low) >> 1);

      if (valid[mid]! < n) {
        low = mid + 1;
      } else {
        high = mid - 1;
      }
    }
    return low;
  };

  let sum = 0n;
  for (const { lhs, rhs } of ranges) {
    sum += prefix_sum[find_pos(rhs + 1n)]! - prefix_sum[find_pos(lhs)]!;
  }
  console.log(sum);
}

main();
