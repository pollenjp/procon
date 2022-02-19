import logging
from dataclasses import dataclass, field
from logging import getLogger
from typing import List, Optional, Set

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)

CONST_MAX_K: int = 20


@dataclass
class Node:
    value: int
    parent: Optional[int] = None
    children: Set[int] = field(default_factory=set)
    inv_sorted_list: Optional[List[int]] = None


def main():

    input_n, input_q = map(int, input().rstrip().split())
    x_list = list(map(int, input().rstrip().split()))
    # logger.info(f"{x_list=}")

    connect_set_list = [set() for _ in range(input_n)]  # 0 origin
    for _ in range(input_n - 1):
        x, y = map(int, input().rstrip().split())
        connect_set_list[x - 1].add(y - 1)
        connect_set_list[y - 1].add(x - 1)

    node_list: List[Node] = [Node(value=x_val) for x_val in x_list]

    def create_node(current_node_idx: int):
        if len(connect_set_list[current_node_idx]) == 0:  # leaf
            return
        current_node: Node = node_list[current_node_idx]
        for child_idx in list(connect_set_list[current_node_idx]):
            connect_set_list[current_node_idx].remove(child_idx)
            connect_set_list[child_idx].remove(current_node_idx)

            current_node.children.add(child_idx)
            node_list[child_idx].parent = current_node_idx

            create_node(child_idx)

    create_node(0)
    # logger.info(f"{node_list=}")

    # recursive sort
    def set_sort_list(current_node_idx: int):
        current_node: Node = node_list[current_node_idx]
        if len(current_node.children) == 0:
            current_node.inv_sorted_list = [current_node.value]
            return

        values: List[int] = [current_node.value]
        for child_idx in current_node.children:
            set_sort_list(child_idx)
            values += node_list[child_idx].inv_sorted_list

        current_node.inv_sorted_list = sorted(values, reverse=True)[:CONST_MAX_K]

    set_sort_list(0)
    # logger.info(f"{node_list=}")

    for _ in range(input_q):
        k, v = map(int, input().rstrip().split())
        node_k: Node = node_list[k - 1]
        print(f"{node_k.inv_sorted_list[v-1]}")


if __name__ == "__main__":
    main()
