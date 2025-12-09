import readline from "node:readline";
import assert from "node:assert";

type Point = { x: number; y: number; z: number };
type Edge = { lhs_idx: number; rhs_idx: number; sq_dist: number };
type Dsu = { p: number; idx: number };

function calc_sq_dist(lhs: Point, rhs: Point): number {
  return (lhs.x - rhs.x) ** 2 + (lhs.y - rhs.y) ** 2 + (lhs.z - rhs.z) ** 2;
}

function get_parent(idx: number, dsus: Dsu[]): number {
  const cur = dsus[idx];
  if (cur.p == cur.idx) {
    return cur.p;
  }
  return (cur.p = get_parent(cur.p, dsus));
}

async function main() {
  const points: Point[] = [];

  for await (let line of readline.createInterface({
    input: process.stdin,
  })) {
    line = line.trim();
    if (!line) {
      continue;
    }

    const match = line.match(/^(?<x>-?\d+),(?<y>-?\d+),(?<z>-?\d+)$/);
    assert(match);
    points.push(
      Object.fromEntries(
        Object.entries(match.groups!).map(([k, v]) => [k, +v]),
      ) as Point,
    );
  }

  const dsus: Dsu[] = [];
  for (let i = 0; i < points.length; i++) {
    dsus.push({ p: i, idx: i });
  }

  const edges: Edge[] = [];
  for (let i = 0; i < points.length; i++) {
    for (let j = i + 1; j < points.length; j++) {
      edges.push({
        lhs_idx: i,
        rhs_idx: j,
        sq_dist: calc_sq_dist(points[i], points[j]),
      });
    }
  }
  edges.sort((lhs, rhs) => lhs.sq_dist - rhs.sq_dist);

  const parents = new Set(Array.from(Array(points.length).keys()));
  for (const edge of edges) {
    let lhs_p = get_parent(edge.lhs_idx, dsus);
    let rhs_p = get_parent(edge.rhs_idx, dsus);

    if (lhs_p == rhs_p) {
      continue;
    }

    if (Math.random() < 0.5) {
      [lhs_p, rhs_p] = [rhs_p, lhs_p];
    }

    parents.delete(dsus[lhs_p].p);
    dsus[lhs_p].p = rhs_p;

    if (parents.size == 1) {
      console.log(points[edge.lhs_idx].x * points[edge.rhs_idx].x);
    }
  }
}

main();
