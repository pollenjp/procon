import itertools
import logging
from logging import getLogger
from typing import List

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_n, input_m = map(int, input().rstrip().split())

    if input_m == 0:
        print("Yes")
        return

    ab_edges = set()
    cd_edges = set()
    for i in range(input_m):
        a, b = map(int, input().rstrip().split())
        if a > b:
            a, b = b, a
        ab_edges.add((a, b))

    for i in range(input_m):
        c, d = map(int, input().rstrip().split())
        if c > d:
            c, d = d, c
        cd_edges.add((c, d))

    def is_same_graph(p_list: List[int]) -> bool:
        for (a, b) in ab_edges:
            p_a, p_b = p_list[a - 1], p_list[b - 1]
            if p_a > p_b:
                p_a, p_b = p_b, p_a
            if (p_a, p_b) not in cd_edges:
                return False
        return True

    flag: bool = False
    for x in itertools.permutations(range(1, input_n + 1)):
        if is_same_graph(x):
            flag = True
            break

    if flag:
        print("Yes")
        return
    print("No")


if __name__ == "__main__":
    main()
