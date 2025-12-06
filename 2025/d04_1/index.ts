import readline from "node:readline";
import assert from "node:assert";

async function main() {
  const rolls: Set<string> = new Set();

  const enc_coords = (y: number, x: number): string => `${y};${x}`;
  const dec_coords = (s: string): [number, number] => {
    const parts = s.split(";");
    assert(parts.length == 2);
    return [+parts[0]!, +parts[1]!];
  };

  let y = 0;
  for await (let line of readline.createInterface({
    input: process.stdin,
  })) {
    line = line.trim();
    if (!line) {
      continue;
    }

    for (let x = 0; x < line.length; x += 1) {
      if (line[x] != "@") {
        continue;
      }
      rolls.add(enc_coords(y, x));
    }
    y += 1;
  }

  let surrounded = 0;
  for (const enc_coord of rolls) {
    const [y, x] = dec_coords(enc_coord);

    let num_around = 0;
    for (let dy = -1; dy <= 1; dy += 1) {
      for (let dx = -1; dx <= 1; dx += 1) {
        num_around += +rolls.has(enc_coords(y + dy, x + dx));
      }
    }

    surrounded += +(num_around <= 4);
  }

  console.log(surrounded);
}

main();
