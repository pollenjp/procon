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


@dataclass
class Node:
    parent: Optional[int] = None
    children: Set[int] = field(default_factory=set)


def main():

    input_n, input_q = map(int, input().rstrip().split())
    x_list = list(map(int, input().rstrip().split()))
    logger.info(f"{x_list=}")

    connect_set_list = [set() for _ in range(input_n)]  # 0 origin
    for _ in range(input_n - 1):
        x, y = map(int, input().rstrip().split())
        connect_set_list[x - 1].add(y - 1)
        connect_set_list[y - 1].add(x - 1)

    node_list: List[Node] = [Node() for i in range(input_n)]

    def create_node(current_node: int):
        if len(connect_set_list[current_node]) == 0:  # leaf
            return
        for child in list(connect_set_list[current_node]):
            connect_set_list[current_node].remove(child)
            connect_set_list[child].remove(current_node)

            node_list[current_node].children.add(child)
            node_list[child].parent = current_node

            create_node(child)

    create_node(current_node=0)
    logger.info(f"{node_list}")

    for _ in range(input_q):
        x, y = map(int, input().rstrip().split())


if __name__ == "__main__":
    main()
