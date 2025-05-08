import re
import sys
from collections import defaultdict
from dataclasses import dataclass

from cachetools import cached
from cachetools.keys import hashkey


@dataclass
class Edge:
    cnt: int
    to_node: str


@cached(cache={}, key=lambda _, node: hashkey(node))
def find_count(tree, node):
    return sum(edge.cnt * (1 + find_count(tree, edge.to_node)) for edge in tree[node])


def main():
    tree = defaultdict(list)
    re_row = re.compile(r"(?P<cnt>\d+) (?P<color>.*?) bags?")
    for line in sys.stdin:
        line = line.strip()
        line = "0 " + line.replace(".", "").replace(",", "").replace("contain", "")

        found = re_row.findall(line)
        lst = tree[found[0][1]]
        for cnt, to_node in found[1:]:
            lst.append(Edge(int(cnt), to_node))

    print(find_count(tree, "shiny gold"))


if __name__ == "__main__":
    main()
