import readline from "node:readline";
import assert from "node:assert";

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
    position += (dir == "R" ? +1 : -1) * parseInt(str_ammount, 10);
    position = ((position % 100) + 100) % 100;
    zeroes += +(position == 0);
  }

  console.log(zeroes);
}

main();
