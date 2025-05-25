import re
import sys
from collections import defaultdict

from cachetools import cached
from cachetools.keys import hashkey


@cached(cache={}, key=lambda _, rule_id: hashkey(rule_id))
def to_regex(rules: dict[int, list[list[int | str]]], rule_id: int) -> str:
    def part_to_regex(part: list[int | str]) -> str:
        return "".join(to_regex(rules, p) if isinstance(p, int) else p for p in part)

    return "(" + "|".join(part_to_regex(part) for part in rules[rule_id]) + ")"


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

    re_rules = re.compile("^" + to_regex(rules, 0) + "$")

    num_ok = 0
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        num_ok += re_rules.match(line) is not None
    print(num_ok)


if __name__ == "__main__":
    main()
