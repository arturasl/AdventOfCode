from __future__ import annotations

from dataclasses import dataclass
from typing import override


@dataclass
class Node:
    label: int
    nxt: Node | None


class List:
    def __init__(self):
        self.label_to_node: dict[int, Node] = {}
        self.first_node: Node | None = None

    @staticmethod
    def from_list(labels: list[int]) -> List:
        result = List()

        assert labels
        result.first_node = Node(labels[0], None)
        result.first_node.nxt = result.first_node
        result.label_to_node[labels[0]] = result.first_node

        prev_label = labels[0]
        for label in labels[1:]:
            result.append_right(prev_label, label)
            prev_label = label
        return result

    def remove_right(self, node_label: int) -> int:
        node = self.label_to_node[node_label]
        assert node
        assert node.nxt
        assert node.nxt.nxt

        if node.nxt == self.first_node:
            self.first_node = node.nxt.nxt

        del_label = node.nxt.label
        del self.label_to_node[del_label]
        node.nxt = node.nxt.nxt
        return del_label

    def append_right(self, node_label: int, new_label: int):
        node = self.label_to_node[node_label]
        assert node
        assert node.nxt

        new_node = Node(new_label, node.nxt)
        assert new_label not in self.label_to_node
        self.label_to_node[new_label] = new_node

        node.nxt = new_node

    def to_list(self) -> list[int]:
        cur_node = self.first_node
        if not cur_node:
            return []

        labels: list[int] = []
        while True:
            assert cur_node
            labels.append(cur_node.label)
            cur_node = cur_node.nxt
            if cur_node == self.first_node:
                break
        return labels

    @override
    def __str__(self):
        return ",".join(str(label) for label in self.to_list())


def solve(cups: list[int], moves: int) -> int:
    ma_cup = max(cups)
    lst = List.from_list(cups)
    cur_label = cups[0]

    for _ in range(moves):
        removed_labels: list[int] = []
        for _ in range(3):
            removed_labels.append(lst.remove_right(cur_label))

        dest_label = (cur_label - 1) % (ma_cup + 1)
        while dest_label not in lst.label_to_node:
            dest_label = (dest_label - 1) % (ma_cup + 1)

        for removed_label in reversed(removed_labels):
            lst.append_right(dest_label, removed_label)

        next_node = lst.label_to_node[cur_label].nxt
        assert next_node
        cur_label = next_node.label

    nxt_to_one = lst.label_to_node[1].nxt
    assert nxt_to_one
    nxt_nxt_to_one = nxt_to_one.nxt
    assert nxt_nxt_to_one

    return nxt_to_one.label * nxt_nxt_to_one.label


def test_first():
    assert solve([3, 8, 9, 1, 2, 5, 4, 6, 7], 10) == 9 * 2


def test_second():
    assert solve([3, 8, 9, 1, 2, 5, 4, 6, 7], 100) == 6 * 7


def test_third():
    assert (
        solve([3, 8, 9, 1, 2, 5, 4, 6, 7] + list(range(10, 1_000_000 + 1)), 10_000_000)
        == 149245887792
    )


def main():
    print(
        solve([9, 4, 2, 3, 8, 7, 6, 1, 5] + list(range(10, 1_000_000 + 1)), 10_000_000)
    )


if __name__ == "__main__":
    main()
