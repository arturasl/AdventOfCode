```.ts
blocks?.[y]?.[x] ?? " "

Array.from(line.matchAll(/\d+/g), (entry) => +entry[0]);
Array.from(line.matchAll(/\d+/g), Number);
Array.from({ length: presses.length }, (_, k) => k);

Array.of(...els);
Array(10).fill(1)
blocks.at(-1)

console.log(`var = ${var}`)

const { dir, n } = line.match(/^(?<dir>[LR])(?<n>\d+)$/).groups;
const [p1, p2] = line.split('-');

const s : Set<number> = new Set();
const m : Map<string, number> = new Map();

const Op = {
  mul: "*",
  add: "+",
} as const;
type Op = (typeof Op)[keyof typeof Op];
const op_funcs: Record<Op, (a: number, b: number) => number> = {
  [Op.mul]: (a: number, b: number) => a * b,
  [Op.add]: (a: number, b: number) => a + b,
};
```
