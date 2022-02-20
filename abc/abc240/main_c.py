import logging
from logging import getLogger
from typing import List, Set, Tuple

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)

const_mod: int = 998244353


def main():

    input_n, input_x = map(int, input().rstrip().split())
    ab_list: List[Tuple[int, int]] = []
    for _ in range(input_n):
        ab_list.append(tuple(map(int, input().rstrip().split())))

    pre_set: Set[int] = set([0])
    for ab in ab_list:
        post_set: Set[int] = set()
        for step in ab:
            for x in pre_set:
                post_set.add(x + step)
        pre_set = post_set
    if input_x in pre_set:
        print("Yes")
        return
    print("No")


if __name__ == "__main__":
    main()
