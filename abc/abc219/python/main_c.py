from logging import getLogger
from typing import Dict, List

import numpy as np

logger = getLogger(__name__)


def main():
    input_n: int = int(input().rstrip())
    x, y = list(map(int, input().rstrip().split(" ")))

    a_list: List[int] = []
    b_list: List[int] = []
    ab_list: List[int] = []
    for i in range(input_n):
        a, b = list(map(int, input().rstrip().split(" ")))
        a_list.append(a)
        b_list.append(b)
        ab_list.append([a, b])

    if sum(a_list) >= x and sum(b_list) >= y:
        print(-1)
        return

    ab_ndarray: np.ndarray = np.array(ab_list, dtype=np.int32)
    logger.warning(ab_ndarray)
    logger.warning(ab_ndarray[ab_ndarray[:, 0].argsort()])


if __name__ == "__main__":
    main()
