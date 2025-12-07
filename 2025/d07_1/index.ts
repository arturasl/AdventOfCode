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

  let splits = 0;
  for (let y = 0; y < map.length - 1; y++) {
    for (let x = 0; x < map[y]!.length; x++) {
      const cur = map[y]![x]!;
      if (cur != "|" && cur != "S") {
        continue;
      }

      const bellow = map[y + 1]![x]!;
      if ([".", "|"].includes(bellow)) {
        map[y + 1]![x] = "|";
        continue;
      }

      assert(["^", "|"].includes(bellow));
      splits += +(bellow == "^");

      const bl = map[y + 1]![x - 1]!;
      assert([".", "|"].includes(bl), bl);
      map[y + 1]![x - 1] = "|";
      assert([".", "|"].includes(map[y + 1]![x + 1]!));
      map[y + 1]![x + 1] = "|";
    }
    console.log(map[y]!.join(""));
  }

  console.log(splits);
}

main();
