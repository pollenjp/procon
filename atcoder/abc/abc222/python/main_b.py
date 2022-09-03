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

    input_n, input_p = map(int, input().rstrip().split())
    input_a_list: List[int] = list(map(int, input().rstrip().split()))

    input_a_ndarray: np.ndarray = np.array(input_a_list)
    cnt: int = np.count_nonzero(input_a_ndarray < input_p)
    print(f"{cnt}")


if __name__ == "__main__":
    main()
