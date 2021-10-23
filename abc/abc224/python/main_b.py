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


def main():

    input_h, input_w = map(int, input().rstrip().split(" "))

    a_tile: List[List[int]] = []
    for i in range(input_h):
        a_tile.append(list(map(int, input().rstrip().split(" "))))

    a_ndarray: np.ndarray = np.array(a_tile)
    logger.info(f"{a_ndarray=}")

    for h1_i in range(input_h - 1):
        for w1_i in range(input_w - 1):
            for h2_i in range(h1_i + 1, input_h):
                for w2_i in range(w1_i + 1, input_w):
                    if not (a_ndarray[h1_i][w1_i] + a_ndarray[h2_i][w2_i]) <= (
                        a_ndarray[h2_i][w1_i] + a_ndarray[h1_i][w2_i]
                    ):
                        print("No")
                        return
    print("Yes")


if __name__ == "__main__":
    main()
