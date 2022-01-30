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

    input_h, input_w = map(int, input().rstrip().split())

    a_list = []
    for i in range(input_h):
        a_list.append(list(map(int, input().rstrip().split())))

    a_ndarray = np.array(a_list)

    b_ndarray = a_ndarray.T

    h, w = b_ndarray.shape[:2]
    for i in range(h):
        print(f"{' '.join(map(str, b_ndarray[i, ::].tolist()))}")


if __name__ == "__main__":
    main()
