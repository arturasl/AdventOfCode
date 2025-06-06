import re
import sys
from collections import defaultdict
from collections.abc import Hashable
from dataclasses import dataclass

from cachetools import cached
from cachetools.keys import hashkey


@dataclass
class Edge:
    cnt: int
    to_node: str


def find_count_hash(_: dict[str, list[Edge]], node: str) -> tuple[Hashable, ...]:
    return hashkey(node)


@cached(cache={}, key=find_count_hash)
def find_count(tree: dict[str, list[Edge]], node: str) -> int:
    return sum(edge.cnt * (1 + find_count(tree, edge.to_node)) for edge in tree[node])


def main():
    tree: dict[str, list[Edge]] = defaultdict(list)
    re_row = re.compile(r"(?P<cnt>\d+) (?P<color>.*?) bags?")
    for line in sys.stdin:
        line = line.strip()
        line = "0 " + line.replace(".", "").replace(",", "").replace("contain", "")

        found: list[tuple[str, str]] = re_row.findall(line)
        lst = tree[found[0][1]]
        for cnt, to_node in found[1:]:
            lst.append(Edge(int(cnt), to_node))

    print(find_count(tree, "shiny gold"))


if __name__ == "__main__":
    main()
