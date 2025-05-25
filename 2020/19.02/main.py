import re
import sys
from collections import defaultdict


def to_regex(
    rules: dict[int, list[list[int | str]]], rule_id: int, cache: dict[int, str]
) -> str:
    if rule_id in cache:
        return cache[rule_id]

    def part_to_regex(part: list[int | str]) -> str:
        return "".join(
            to_regex(rules, p, cache) if isinstance(p, int) else p for p in part
        )

    cache[rule_id] = (
        "(" + "|".join(part_to_regex(part) for part in rules[rule_id]) + ")"
    )
    return cache[rule_id]


def main():
    rules: dict[int, list[list[int | str]]] = defaultdict(list)

    for line in sys.stdin:
        line = line.strip()
        if not line:
            break
        name, str_rule = line.split(": ")
        name = int(name)
        str_parts = str_rule.split(" | ")

        for str_part in str_parts:
            rules[name].append(
                [int(n) if n[0] != '"' else n[1:-1] for n in str_part.split(" ")]
            )

    assert len(rules[0]) == 1

    checks: list[str | None] = [
        line for line in [line.strip() for line in sys.stdin] if line
    ]

    for reps in range(1, max(len(c) for c in checks if c) // 2 + 1):
        cache: dict[int, str] = {}
        cache[8] = to_regex(rules, 42, cache) + "+"
        cache[11] = (
            to_regex(rules, 42, cache) * reps + to_regex(rules, 31, cache) * reps
        )
        re_rules = re.compile("^" + to_regex(rules, 0, cache) + "$")

        for i, c in enumerate(checks):
            if not c or not re_rules.match(c):
                continue
            checks[i] = None

    print(sum(c is None for c in checks))


if __name__ == "__main__":
    main()
