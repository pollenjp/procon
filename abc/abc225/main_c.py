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


def detect(b_ndarray: np.ndarray, col_offset: int) -> bool:
    n, m = b_ndarray.shape[:2]

    b2_ndarray: np.ndarray = b_ndarray - np.tile(np.arange(m) + col_offset + 1, (n, 1))

    logger.info(b2_ndarray)

    for i in range(1, b2_ndarray.shape[1]):
        flag: bool = np.all(b2_ndarray[:, 0] == b2_ndarray[:, i])
        if not flag:
            return False

    logger.info(f"{b2_ndarray[:, 0]=}")
    logger.info(f"{b2_ndarray[:, 0] % 7=}")
    if np.all(b2_ndarray[:, 0] % 7 == 0):
        return True
    return False


def main():

    input_n, input_m = map(int, input().rstrip().split(" "))

    b_array: List[List[int]] = []
    for _ in range(input_n):
        b_array.append(list(map(int, input().rstrip().split(" "))))

    b_ndarray: np.ndarray = np.array(b_array)

    for col_offset in range(7 - input_m + 1):
        if detect(b_ndarray, col_offset):
            print("Yes")
            return
    print("No")


if __name__ == "__main__":
    main()
