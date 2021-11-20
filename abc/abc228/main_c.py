import logging
from logging import getLogger
from typing import List

import numpy as np

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def calc_area(a: int, b: int) -> int:
    return 4 * a * b + 3 * a + 3 * b


def main():
    input_n, input_k = map(int, input().rstrip().split())
    input_p_point_list = [list(map(int, input().rstrip().split())) for _ in range(input_n)]

    p_point_ndarray = np.array(input_p_point_list)

    x = np.sum(p_point_ndarray, axis=1)
    # logger.info(f"{x=}")
    x_sort = np.sort(x)[::-1]
    # logger.info(f"{x_sort=}")

    base = x_sort[input_k - 1]
    for i, p_point_sum in enumerate(x):
        # logger.info(f"{i=}, {p_point_sum=}, {base=}")
        if base >= p_point_sum:
            if base <= p_point_sum + 300:
                print("Yes")
                continue
            print("No")
            continue
        print("Yes")


if __name__ == "__main__":
    main()
