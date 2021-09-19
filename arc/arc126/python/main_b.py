import logging
import textwrap
from logging import getLogger
from typing import List, Tuple

import numpy as np

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)

logger.info(
    textwrap.indent(
        textwrap.dedent(
            f"""
            sample
            """.rstrip()
        ),
        " " * 4,
    )
)


def main():

    input_n, input_m = map(int, input().split())
    ab_list: List[Tuple[int, int]] = []
    for i in range(input_m):
        ab_list.append(tuple(map(int, input().split())))

    cross_points_list: List[int] = [[] for _ in range(input_m)]
    point_cnt_list: List[int] = [0 for _ in range(input_m)]

    logger.info(f"{ab_list=}")

    for i in range(input_m):
        for j in range(i, input_m):
            a1, b1 = ab_list[i]
            a2, b2 = ab_list[j]
            if a1 == a2 and b1 == b2:  # これは前提で起こらない
                continue
            if (a1 - a2) * (b1 - b2) <= 0:
                cross_points_list[i].append(j)
                cross_points_list[j].append(i)
                point_cnt_list[i] += 1
                point_cnt_list[j] += 1

    cnt: int = 0
    point_cnt_ndarray: np.ndarray = np.array(point_cnt_list)
    while point_cnt_ndarray.sum() > 0:
        i = np.argmax(point_cnt_ndarray)
        for j in cross_points_list[i]:
            cross_points_list[j].remove(i)
            point_cnt_ndarray[j] -= 1
            if point_cnt_ndarray[j] < 0:
                raise Exception()
        point_cnt_ndarray[i] = 0
        cnt += 1
    print(len(point_cnt_ndarray) - cnt)


if __name__ == "__main__":
    main()
