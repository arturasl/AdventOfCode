import re
import sys
from collections import defaultdict
from dataclasses import dataclass


@dataclass
class Edge:
    cnt: int
    to_node: str


def main():
    parents = defaultdict(list)
    re_row = re.compile(r"(?P<cnt>\d+) (?P<color>.*?) bags?")
    for line in sys.stdin:
        line = line.strip()
        line = "0 " + line.replace(".", "").replace(",", "").replace("contain", "")

        found = re_row.findall(line)
        parent = found[0][1]
        for cnt, to_node in found[1:]:
            parents[to_node].append(Edge(cnt, parent))

    visited = set([])
    queue = ["shiny gold"]
    while queue:
        cur = queue.pop()
        if cur in visited:
            continue
        visited.add(cur)

        for back_edge in parents[cur]:
            queue.append(back_edge.to_node)

    print(len(visited) - 1)


if __name__ == "__main__":
    main()
