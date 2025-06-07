from __future__ import annotations


class Node:
    def __init__(self, label: int, nxt: Node | None = None):
        self.label: int = label
        self.nxt: Node = nxt or self


class List:
    def __init__(self):
        self.label_to_node: dict[int, Node] = {}
        self.first_node: Node | None = None

    @staticmethod
    def from_list(labels: list[int]) -> List:
        result = List()

        assert labels
        result.first_node = Node(labels[0])
        result.label_to_node[labels[0]] = result.first_node

        prev_label = labels[0]
        for label in labels[1:]:
            result.append_right(prev_label, label)
            prev_label = label
        return result

    def remove_right(self, node_label: int) -> int:
        node = self.label_to_node[node_label]
        assert node

        if node.nxt == self.first_node:
            self.first_node = node.nxt.nxt

        del_label = node.nxt.label
        del self.label_to_node[del_label]
        node.nxt = node.nxt.nxt
        return del_label

    def append_right(self, node_label: int, new_label: int):
        node = self.label_to_node[node_label]
        assert node

        new_node = Node(new_label, node.nxt)
        assert new_label not in self.label_to_node
        self.label_to_node[new_label] = new_node

        node.nxt = new_node


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
        cur_label = next_node.label

    nxt_to_one = lst.label_to_node[1].nxt
    nxt_nxt_to_one = nxt_to_one.nxt

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
