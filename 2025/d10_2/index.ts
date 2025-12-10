import readline from "node:readline";
import assert from "node:assert";
import { init as initZ3 } from "z3-solver";

type Z3Instance = Awaited<
  ReturnType<Awaited<ReturnType<typeof initZ3>>["Context"]>
>;

async function solve(z3: Z3Instance, jolts: number[], presses: number[][]) {
  const { Optimize, Int } = z3;

  const ps = Array.from({ length: presses.length }, (_, k) =>
    Int.const(`p${k}`),
  );

  const opt = new Optimize();

  for (let i = 0; i < jolts.length; i += 1) {
    opt.add(
      presses
        .entries()
        .filter(([_, p]) => p.some((b) => b == i))
        .map(([k]) => ps[k])
        .reduce((acc, cur) => acc.add(cur))
        .eq(jolts[i]),
    );
  }

  for (const p of ps) {
    opt.add(p.ge(0));
  }

  const total = ps.reduce((acc, cur) => acc.add(cur));
  opt.minimize(total);
  assert((await opt.check()) == "sat");
  return +opt.model().eval(total).toString();
}

async function main() {
  const { Context } = await initZ3();
  const z3 = Context("main" as string);

  let result = 0;
  for await (let line of readline.createInterface({
    input: process.stdin,
  })) {
    line = line.trim();
    if (!line) {
      continue;
    }

    const match = line.match(
      /^(?<buttons>\[[\.#]*\])(?<presses>(?: \([\d,]*\))*)* (?<jolts>\{[\d,]*\})$/,
    );
    assert(match);
    assert(match.groups);

    const presses = match.groups.presses
      .trim()
      .split(" ")
      .map((p) => p.slice(1, -1).split(",").map(Number));

    const jolts = match.groups.jolts.trim().slice(1, -1).split(",").map(Number);

    result += await solve(z3, jolts, presses);
  }

  console.log(result);
}

main();
