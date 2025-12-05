import readline from "node:readline";
import assert from "node:assert";

async function main() {
  const triggers: [pos: number, inc: number][] = [];
  const ids: number[] = [];

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
      triggers.push([+start, 1]);
      triggers.push([+end, -1]);
    } else {
      assert(line.match(/^-?\d+$/));
      ids.push(+line);
    }
  }

  triggers.sort(([lhs], [rhs]) => lhs - rhs);
  ids.sort((lhs, rhs) => lhs - rhs);

  let sum = 0;
  let in_range_ids = 0;
  let trigger_idx = 0;
  for (const id of ids) {
    for (
      ;
      trigger_idx < triggers.length && id > triggers[trigger_idx]![0];
      trigger_idx++
    ) {
      sum += triggers[trigger_idx]![1];
    }
    in_range_ids += +(sum > 0);
  }
  console.log(in_range_ids);
}

main();
