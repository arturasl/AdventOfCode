import readline from "node:readline";
import assert from "node:assert";

async function main() {
  const map: string[][] = [];
  for await (let line of readline.createInterface({
    input: process.stdin,
  })) {
    line = line.trim();
    if (!line) {
      continue;
    }

    map.push(Array.from("." + line + "."));
  }
  map.push(Array(map.at(-1)!.length).fill("."));

  const timelines: number[][] = [];
  for (let y = 0; y < map.length; y++) {
    timelines.push(Array(map[y].length).fill(0));
  }

  for (let y = 0; y < map.length - 1; y++) {
    for (let x = 0; x < map[y].length; x++) {
      const cur = timelines[y][x] + +(map[y][x] == "S");
      if (!cur) {
        continue;
      }

      const bellow = map[y + 1][x];
      if (bellow == ".") {
        timelines[y + 1][x] += cur;
        continue;
      }

      assert(bellow == "^");

      assert(map[y + 1][x - 1] == ".");
      timelines[y + 1][x - 1] += cur;
      assert(map[y + 1][x + 1] == ".");
      timelines[y + 1][x + 1] += cur;
    }
  }

  console.log(timelines.at(-1)!.reduce((a, b) => a + b));
}

main();
