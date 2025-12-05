import readline from "node:readline";
import assert from "node:assert";

async function main() {
  type Inc = [pos: bigint, inc: bigint];
  const triggers: Inc[] = [];

  for await (let line of readline.createInterface({
    input: process.stdin,
  })) {
    line = line.trim();
    if (!line) {
      continue;
    }

    const match_range = line.match(/^(?<start>-?\d+)-(?<end>-?\d+)$/);
    if (match_range) {
      const { start, end } = match_range.groups as {
        start: string;
        end: string;
      };
      triggers.push([BigInt(start), 1n]);
      triggers.push([BigInt(end) + 1n, -1n]);
    }
  }

  triggers.sort(([lhs], [rhs]) => {
    if (lhs < rhs) {
      return -1;
    }
    if (lhs > rhs) {
      return 1;
    }
    return 0;
  });

  const uniq_triggers: Inc[] = [];
  for (let idx = 0; idx < triggers.length; ) {
    let [pos, inc] = triggers[idx]!;
    idx += 1;
    while (idx < triggers.length && pos == triggers[idx]![0]) {
      inc += triggers[idx]![1];
      idx += 1;
    }

    if (inc) {
      uniq_triggers.push([pos, inc]);
    }
  }

  assert(uniq_triggers.length == new Set(uniq_triggers.map((pos) => pos)).size);

  let result = 0n;
  let prev_sum = 0n;
  let prev_start: bigint | null = null;
  for (const [pos, inc] of uniq_triggers) {
    if (prev_sum == 0n) {
      assert(inc >= 1);
      assert(prev_start == null);
      prev_sum += inc;
      prev_start = pos;
    } else {
      prev_sum += inc;
      if (prev_sum == 0n) {
        assert(inc <= -1);
        assert(prev_start != null);
        result += pos - prev_start;
        prev_start = null;
      }
    }
  }
  assert(prev_sum == 0n);

  console.log(result.toString());
}

main();
