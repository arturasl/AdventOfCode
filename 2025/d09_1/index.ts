import readline from "node:readline";
import assert from "node:assert";

type Point = { x: number; y: number };

async function main() {
  const points: Point[] = [];
  for await (let line of readline.createInterface({
    input: process.stdin,
  })) {
    line = line.trim();
    if (!line) {
      continue;
    }

    const match = line.match(/^(?<x>-?\d+),(?<y>-?\d+)$/);
    assert(match);
    points.push(
      Object.fromEntries(
        Object.entries(match.groups!).map(([k, v]) => [k, +v]),
      ) as Point,
    );
  }

  let max_area = 0;
  for (let i = 0; i < points.length; i++) {
    for (let j = i + 1; j < points.length; j++) {
      max_area = Math.max(
        max_area,
        (Math.abs(points[i].x - points[j].x) + 1) *
          (Math.abs(points[i].y - points[j].y) + 1),
      );
    }
  }

  console.log(max_area);
}

main();
