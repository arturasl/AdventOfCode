import readline from "node:readline";
import assert from "node:assert";

async function main() {
  const graph: Map<string, string[]> = new Map();
  for await (let line of readline.createInterface({
    input: process.stdin,
  })) {
    line = line.trim();
    if (!line) {
      continue;
    }

    const parts = line.split(":");
    const from = parts[0].trim();
    const to = parts[1]
      .split(" ")
      .map((s) => s.trim())
      .filter((s) => s);

    assert(!graph.has(from));
    graph.set(from, to);
  }

  function count_paths(from: string, to: string): number {
    const cache: Map<string, number> = new Map();
    function count(cur: string): number {
      if (cur == to) {
        return 1;
      }

      const prev_result = cache.get(cur);
      if (prev_result != null) {
        return prev_result;
      }

      const other = graph.get(cur) ?? [];
      const next_result = other.map((t) => count(t)).reduce((a, b) => a + b, 0);
      cache.set(cur, next_result);
      return next_result;
    }

    return count(from);
  }

  console.log(
    count_paths("svr", "dac") *
      count_paths("dac", "fft") *
      count_paths("fft", "out") +
      count_paths("svr", "fft") *
        count_paths("fft", "dac") *
        count_paths("dac", "out"),
  );
}

main();
