import readline from "node:readline";
import assert from "node:assert";

async function main() {
  const rolls = new Set();

  const enc_coords = (y, x) => `${y};${x}`;
  const dec_coords = (s) => {
    const parts = s.split(";");
    assert(parts.length == 2);
    return [parseInt(parts[0], 10), parseInt(parts[1], 10)];
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

  let around = new Map();
  const to_remove = new Set();

  for (const enc_coord of rolls.values()) {
    const [y, x] = dec_coords(enc_coord);
    around.set(enc_coord, new Set());
    for (let dy = -1; dy <= 1; dy += 1) {
      for (let dx = -1; dx <= 1; dx += 1) {
        const [ny, nx] = [y + dy, x + dx];
        if (ny == y && nx == x) {
          continue;
        }
        if (rolls.has(enc_coords(ny, nx))) {
          around.get(enc_coord).add(enc_coords(ny, nx));
        }
      }
    }

    if (around.get(enc_coord).size < 4) {
      to_remove.add(enc_coord);
    }
  }

  let removed = 0;
  while (to_remove.size) {
    const enc_cur = to_remove.values().next().value;
    to_remove.delete(enc_cur);
    removed += 1;

    for (const other of around.get(enc_cur).values()) {
      around.get(other).delete(enc_cur);
      if (around.get(other).size < 4) {
        to_remove.add(other);
      }
    }
  }

  console.log(removed);
}

main();
