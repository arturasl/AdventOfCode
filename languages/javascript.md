```.js
blocks?.[y]?.[x] ?? " "
Array.from(line.matchAll(/\d+/g), (entry) => +entry[0];);

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
