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

const_angle_mod = 360


def main():

    input_n = int(input().rstrip())

    input_a_list: List[int] = list(map(int, input().rstrip().split()))

    cut_angle_list: List[int] = [0]
    current_angle = 0
    for a in input_a_list:
        current_angle += a
        current_angle %= const_angle_mod
        cut_angle_list.append(const_angle_mod - current_angle)

    cut_angle_list.sort()

    ans: int = 0
    for i, j in zip(cut_angle_list[:-1], cut_angle_list[1:]):
        ans = max(ans, j - i)

    ans = max(ans, const_angle_mod - (cut_angle_list[0] + cut_angle_list[-1]))

    print(f"{ans}")


if __name__ == "__main__":
    main()
