import logging
from dataclasses import dataclass, field
from logging import getLogger
from typing import List, Optional, Set, Tuple

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


@dataclass
class Node:
    value: int
    parent: Optional[int] = None
    parent_min: Optional[int] = None  # minにすべき値
    children: Set[int] = field(default_factory=set)
    lr: Optional[Tuple[int, int]] = None


def main():

    input_n = int(input().rstrip())
    connect_set_list = [set() for _ in range(input_n)]  # 0 origin
    for _ in range(input_n - 1):
        u, v = map(int, input().rstrip().split())
        u, v = u - 1, v - 1
        connect_set_list[u].add(v)
        connect_set_list[v].add(u)

    node_list: List[Node] = [Node(value=i) for i in range(input_n)]  # 0 origin
    node_list[0].parent_min = 1

    def create_node(current_node_idx: int):
        if len(connect_set_list[current_node_idx]) == 0:  # leaf
            return
        for child_idx in list(connect_set_list[current_node_idx]):
            connect_set_list[current_node_idx].remove(child_idx)
            connect_set_list[child_idx].remove(current_node_idx)

            node_list[current_node_idx].children.add(child_idx)
            node_list[child_idx].parent = current_node_idx

            create_node(child_idx)

    create_node(0)
    # logger.info(f"{node_list=}")

    # recursive
    def set_lr(current_node_idx: int):
        current_node: Node = node_list[current_node_idx]

        if len(current_node.children) == 0:  # leaf
            current_node.lr = (current_node.parent_min, current_node.parent_min)
            return

        min_val: int = current_node.parent_min
        for child_idx in current_node.children:
            child_node: Node = node_list[child_idx]

            child_node.parent = current_node_idx
            child_node.parent_min = max(current_node.parent_min, min_val)

            set_lr(child_idx)

            min_val = child_node.lr[1] + 1
        current_node.lr = (current_node.parent_min, child_node.lr[1])  # 最後のchildの R が最大

    set_lr(0)
    # logger.info(f"{node_list=}")

    for node in node_list:
        print(f"{node.lr[0]} {node.lr[1]}")


if __name__ == "__main__":
    main()
