import readline from "node:readline";
import assert from "node:assert";

function mod(x: number, m: number): number {
  return ((x % m) + m) % m;
}

function div(a: number, b: number): number {
  return Math.trunc(a / b);
}

async function main() {
  let position = 50;
  let zeroes = 0;
  for await (const line of readline.createInterface({
    input: process.stdin,
  })) {
    const match = line.match(/^(?<dir>[LR])(?<str_ammount>\d+)$/);
    assert(match, `While parsing ${line}`);
    const { dir, str_ammount } = match.groups as {
      dir: string;
      str_ammount: string;
    };

    const next_position =
      position + (dir == "R" ? +1 : -1) * parseInt(str_ammount, 10);

    if (next_position % 100 == 0) {
      zeroes += 1;
    }

    let [lhs, rhs] = [position, next_position];
    if (rhs < lhs) {
      [lhs, rhs] = [rhs, lhs];
    }

    lhs = lhs + 100 - mod(lhs, 100);

    if (lhs < rhs) {
      zeroes += 1 + div(rhs - lhs - 1, 100);
    }

    position = mod(next_position, 100);
  }

  console.log(zeroes);
}

main();
