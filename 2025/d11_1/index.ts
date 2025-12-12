import readline from "node:readline";
import assert from "node:assert";

type Node = { paths: number; to: string[] };

async function main() {
  const graph: Map<string, Node> = new Map();
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
    graph.set(from, { paths: -1, to });
  }

  function count_paths(cur: string): number {
    if (cur == "out") {
      return 1;
    }

    const node = graph.get(cur);
    assert(node);
    if (node.paths != -1) {
      return node.paths;
    }

    node.paths = 0;
    return (node.paths = node.to
      .map((t) => count_paths(t))
      .reduce((a, b) => a + b));
  }

  console.log(count_paths("you"));
}

main();
