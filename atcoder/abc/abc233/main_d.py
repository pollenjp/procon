import logging
from logging import getLogger

import numpy as np

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():
    # MIN_VALUE: int = int(-1e9)

    input_n, input_k = map(int, input().rstrip().split())
    a_list = list(map(int, input().rstrip().split()))
    # a_list = [a - MIN_VALUE for a in a_list]  # 正のみにする
    a_ndarray = np.array(a_list, dtype=np.int64)
    # logger.info(f"{a_ndarray=}")
    a_accum = np.cumsum(a_ndarray, dtype=np.int64)
    # logger.info(f"{a_accum=}")
    cnt: int = (a_accum == input_k).sum()
    for i in range(1, input_n):
        # tmp = a_accum[i:] - a_accum[0 : input_n - i]
        # logger.info(f"{tmp=}")
        cnt += (a_accum[i:] - a_accum[0 : input_n - i] == input_k).sum()

    print(f"{cnt}")


if __name__ == "__main__":
    main()
