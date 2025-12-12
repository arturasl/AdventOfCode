import readline from "node:readline";

async function main() {
  const counts = { ok: 0, maybe: 0, nok: 0 };
  for await (let line of readline.createInterface({
    input: process.stdin,
  })) {
    line = line.trim();
    if (!line) {
      continue;
    }

    const parts = line.split(": ");
    if (parts.length != 2) {
      continue;
    }

    const [h, w] = parts[0].split("x").map(Number);
    const squares = parts[1]
      .split(" ")
      .map(Number)
      .reduce((a, b) => a + b);

    if (Math.floor(w / 3) * Math.floor(h / 3) >= squares) {
      counts.ok += 1;
    } else if (w * h <= 7 * squares) {
      counts.nok += 1;
    } else {
      counts.maybe += 1;
    }
  }

  console.log(counts);
}

main();
