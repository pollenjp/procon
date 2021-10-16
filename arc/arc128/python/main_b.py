import logging
from logging import getLogger
from typing import List, Tuple

import numpy as np

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)

CONST_MAX: int = 1e9 + 1


def solve(num_r: int, num_g: int, num_b: int) -> int:
    num_list: List[int] = [num_r, num_g, num_b]

    unable_flag: bool = True

    pair_list: List[Tuple[int, int]] = []
    for num1, num2 in zip(num_list, num_list[1:] + [num_list[0]]):
        if (num1 - num2) % 3 == 0:
            unable_flag = False
            pair_list.append((num1, num2))
            # logger.info(f"{num1=}, {num2=} => {num1 - num2=}")

    if unable_flag:
        return -1

    # logger.info(f"{pair_list=}")
    target_idx: int = 0
    if len(pair_list) > 1:
        target_idx = np.argmin([num1 + num2 for (num1, num2) in pair_list])
    _pair = pair_list[target_idx]
    # logger.info(f"{_pair}")
    # logger.info(f"{sorted(_pair)}")
    num_min, num_max = sorted(_pair)
    k = (num_max - num_min) // 3
    left = num_max - k

    cnt: int = left + k

    return cnt


def main():
    input_t: int = int(input())
    for t_idx in range(input_t):
        rgb_list: List[int] = list(map(int, input().split()))
        # logger.info(f"{t_idx=}")
        print(f"{solve(*rgb_list)}")


if __name__ == "__main__":
    main()
