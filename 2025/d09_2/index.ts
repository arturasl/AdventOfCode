import readline from "node:readline";
import assert from "node:assert";

type Point = { x: number; y: number };
type Rect = { area: number; bl: Point; tr: Point };
type Compression = {
  points: Point[];
  compress: (p: Point) => Point;
  uncompress: (p: Point) => Point;
};

function create_covering_matrix<T>(points: Point[], fill_val: T): T[][] {
  let ma_y = Math.max(...points.map((p) => p.y));
  let ma_x = Math.max(...points.map((p) => p.x));

  return Array(ma_y + 2)
    .fill(0)
    .map(() =>
      Array(ma_x + 2)
        .fill(0)
        .map(() => structuredClone(fill_val)),
    );
}

function show_matrix<T>(matrix: T[][], sep = "") {
  for (const row of matrix) {
    console.log(row.join(sep));
  }
}

function show_points(points: Point[]) {
  let matrix: string[][] = create_covering_matrix(points, ".");
  for (const point of points) {
    matrix[point.y][point.x] = "#";
  }
  show_matrix(matrix);
}

function compress_points(points: Point[]): Compression {
  const create_mapping = (get_dim: (p: Point) => number) => {
    const uniq_dims: Set<number> = new Set(points.map(get_dim));
    const uniq_expanded_dims: Set<number> = new Set();
    for (const dim of uniq_dims) {
      uniq_expanded_dims.add(dim - 1);
      uniq_expanded_dims.add(dim);
      uniq_expanded_dims.add(dim + 1);
    }
    const entries = Array.from(uniq_expanded_dims)
      .sort((a, b) => a - b)
      .entries()
      .toArray();
    return {
      orig_to_compressed: new Map(entries.map(([idx, v]) => [v, idx])),
      compressed_to_orig: new Map(entries),
    };
  };
  const ys = create_mapping((p) => p.y * 2);
  const xs = create_mapping((p) => p.x * 2);

  const compress = (p: Point) =>
    ({
      y: ys.orig_to_compressed.get(p.y * 2),
      x: xs.orig_to_compressed.get(p.x * 2),
    }) as Point;
  const uncompress = (p: Point) =>
    ({
      y: ys.compressed_to_orig.get(p.y / 2),
      x: xs.compressed_to_orig.get(p.x / 2),
    }) as Point;

  return {
    points: points.map(compress),
    compress,
    uncompress,
  };
}

function create_filled_interrior(points: Point[]) {
  const matrix: string[][] = create_covering_matrix(points, ".");
  for (let i = 0; i < points.length; i += 1) {
    const cur = points[i];
    const next = points[(i + 1) % points.length];
    if (cur.x == next.x) {
      const [mi_y, ma_y] = [Math.min(cur.y, next.y), Math.max(cur.y, next.y)];
      for (let y = mi_y; y <= ma_y; y += 1) {
        matrix[y][cur.x] = "#";
      }
    } else {
      const [mi_x, ma_x] = [Math.min(cur.x, next.x), Math.max(cur.x, next.x)];
      for (let x = mi_x; x <= ma_x; x += 1) {
        matrix[cur.y][x] = "#";
      }
    }
  }
  assert(matrix[0][0] == ".");

  const stack: Point[] = [{ y: 0, x: 0 }];
  matrix[0][0] = "o";
  while (stack.length) {
    const cur = stack.pop()!;
    for (let dy = -1; dy <= 1; dy += 1) {
      for (let dx = -1; dx <= 1; dx += 1) {
        if (dy == 0 && dx == 0) {
          continue;
        }
        if (dy != 0 && dx != 0) {
          continue;
        }
        const [ny, nx] = [cur.y + dy, cur.x + dx];
        if (ny < 0 || nx < 0) {
          continue;
        }
        if (ny >= matrix.length || nx >= matrix[ny].length) {
          continue;
        }
        if (matrix[ny][nx] != ".") {
          continue;
        }
        matrix[ny][nx] = "o";
        stack.push({ y: ny, x: nx });
      }
    }
  }

  for (let y = 0; y < matrix.length; y++) {
    for (let x = 0; x < matrix[y].length; x++) {
      switch (matrix[y][x]) {
        case "o":
          matrix[y][x] = ".";
          break;
        case ".":
          matrix[y][x] = "#";
          break;
        case "#":
          break;
        default:
          assert(false, `Found: '${matrix[y][x]}'`);
      }
    }
  }

  return matrix;
}

function get_priotized_rectangles(points: Point[]): Rect[] {
  let rects: Rect[] = [];
  for (let i = 0; i < points.length; i++) {
    for (let j = i + 1; j < points.length; j++) {
      const bl: Point = {
        x: Math.min(points[i].x, points[j].x),
        y: Math.min(points[i].y, points[j].y),
      };
      const tr: Point = {
        x: Math.max(points[i].x, points[j].x),
        y: Math.max(points[i].y, points[j].y),
      };
      rects.push({
        area: (tr.x - bl.x + 1) * (tr.y - bl.y + 1),
        bl,
        tr,
      });
    }
  }

  rects.sort((a, b) => b.area - a.area);
  return rects;
}

function bin_search_ge(nums: number[], val: number): number {
  let low = 0;
  let high = nums.length;
  while (low < high) {
    const mid = low + ((high - low) >> 1);
    if (nums[mid] >= val) {
      high = mid;
    } else {
      low = mid + 1;
    }
  }
  return high;
}

function find_first_covered_rect(
  rects: Rect[],
  comp_matrix: string[][],
  compression: Compression,
): Rect | null {
  const nonfilled_y_at_x: number[][] = Array(comp_matrix[0].length)
    .fill(0)
    .map(() => []);
  for (let x = 0; x < comp_matrix[0].length; x++) {
    for (let y = 0; y < comp_matrix.length; y++) {
      if (comp_matrix[y][x] == ".") {
        nonfilled_y_at_x[x].push(y);
      }
    }
  }

  for (const rect of rects) {
    const comp_bl = compression.compress(rect.bl);
    const comp_tr = compression.compress(rect.tr);

    let is_filled = true;
    for (let x = comp_bl.x; x <= comp_tr.x && is_filled; x += 1) {
      const first_incl_idx = bin_search_ge(nonfilled_y_at_x[x], comp_bl.y);
      const first_exl_idx = bin_search_ge(nonfilled_y_at_x[x], comp_tr.y + 1);
      assert(first_incl_idx <= first_exl_idx);
      is_filled = is_filled && first_incl_idx == first_exl_idx;
    }
    if (is_filled) {
      return rect;
    }
  }

  return null;
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

    const match = line.match(/^(?<x>-?\d+),(?<y>-?\d+)$/);
    assert(match);
    points.push(
      Object.fromEntries(
        Object.entries(match.groups!).map(([k, v]) => [k, +v]),
      ) as Point,
    );
  }
  // show_points(points);

  const compression: Compression = compress_points(points);
  // show_points(compression.points);

  const comp_matrix: string[][] = create_filled_interrior(compression.points);
  // show_matrix(comp_matrix, "");

  const rects: Rect[] = get_priotized_rectangles(points);
  // console.log(rects);

  const best_rect: Rect | null = find_first_covered_rect(
    rects,
    comp_matrix,
    compression,
  );
  assert(best_rect);
  console.log(best_rect.area);
}

main();
