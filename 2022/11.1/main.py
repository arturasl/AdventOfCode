import sys
import re
import operator
import functools

class Monkey:
    def __str__(self):
        return f'Monkey {self.idx}, items: {self.items}, if ok: {self.if_ok}, if nok: {self.if_nok}, inspections: {self.inspections}'

def main():
    monkeys = []
    it = iter(sys.stdin)
    def match(regex):
        while True:
            line = next(it).strip()
            if not line:
                continue
            m = re.match(regex, line)
            assert m, f'regexp: "{regex}" did not match "{line}"'
            return m.groups()

    while True:
        monkey = Monkey()
        try:
            m = match(r'^Monkey (\d+):$')
        except StopIteration:
            break
        assert int(m[0]) == len(monkeys)
        monkey.idx = int(m[0])

        m = match(r'Starting items: ((:?\d+, )*\d+)$')
        monkey.items = [int(x.strip()) for x in m[0].split(',')]

        m = match(r'Operation: new = (\d+|old) (\+|\*) (\d+|old)$')
        monkey.run_operation = lambda old, m=m: {'+': operator.add, '*': operator.mul}[m[1]](
            *[old if m[i] == 'old' else int(m[i]) for i in [0, 2]]
        )

        m = match(r'Test: divisible by (\d+)$')
        monkey.check_test = lambda x, m=m: x % int(m[0]) == 0

        m = match(r'If true: throw to monkey (\d+)$')
        monkey.if_ok = int(m[0])
        assert monkey.if_ok != monkey.idx

        m = match(r'If false: throw to monkey (\d+)$')
        monkey.if_nok = int(m[0])
        assert monkey.if_nok != monkey.idx

        monkey.inspections = 0

        monkeys.append(monkey)

    for _ in range(20):
        for monkey in monkeys:
            for item in monkey.items:
                w = monkey.run_operation(item) // 3
                monkeys[[monkey.if_nok, monkey.if_ok][monkey.check_test(w)]].items.append(w)
                monkey.inspections += 1
            monkey.items = []

    for monkey in monkeys:
        print(monkey)

    print(functools.reduce(lambda o, c: c.inspections * o, sorted(monkeys, key=lambda x: -x.inspections)[:2], 1))

if __name__ == "__main__":
    main()
