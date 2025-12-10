import readline from "node:readline";
import assert from "node:assert";
import { MinPriorityQueue } from "@datastructures-js/priority-queue";

function solve(buttons: boolean[], presses: number[][]): number {
  const buttons_to_str = (b: boolean[]) =>
    b.map((b) => (b ? "#" : ".")).join("");
  const visited: Set<string> = new Set();

  type State = { dist: number; buttons: boolean[] };
  const queue = new MinPriorityQueue<State>((el) => el.dist);
  queue.push({ dist: 0, buttons });
  visited.add(buttons_to_str(buttons));

  while (!queue.isEmpty()) {
    const cur: State = queue.pop()!;
    if (cur.buttons.reduce((a, b) => a && !b, true)) {
      return cur.dist;
    }

    for (const seq of presses) {
      const new_buttons = structuredClone(cur.buttons);
      for (const press of seq) {
        new_buttons[press] = !new_buttons[press];
      }

      const new_buttons_str = buttons_to_str(new_buttons);
      if (visited.has(new_buttons_str)) {
        continue;
      }
      visited.add(new_buttons_str);

      queue.push({ dist: cur.dist + 1, buttons: new_buttons });
    }
  }

  assert(false);
}

async function main() {
  let result = 0;
  for await (let line of readline.createInterface({
    input: process.stdin,
  })) {
    line = line.trim();
    if (!line) {
      continue;
    }

    const match = line.match(
      /^(?<buttons>\[[\.#]*\])(?<presses>(?: \([\d,]*\))*)* (?<jolts>\{[\d,]*\})$/,
    );
    assert(match);
    assert(match.groups);

    const buttons = Array.from(match.groups.buttons.slice(1, -1)).map(
      (b) => b == "#",
    );

    const presses = match.groups.presses
      .trim()
      .split(" ")
      .map((p) =>
        p
          .slice(1, -1)
          .split(",")
          .map((n) => +n),
      );

    result += solve(buttons, presses);
  }

  console.log(result);
}

main();
