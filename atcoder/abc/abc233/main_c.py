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

    input_n, input_x = map(int, input().rstrip().split())
    rows = []
    for i in range(input_n):
        rows.append(list(map(int, input().rstrip().split())))

    array_list = [np.array(row[1:], dtype=np.uint64) for row in rows]

    values = np.array([[1]], dtype=np.uint64)
    for arr in array_list:
        values = np.matmul(values, arr.reshape(1, -1)).flatten()
        values = values[values <= input_x].reshape(-1, 1)
    print(f"{(values.flatten() == input_x).sum()}")


if __name__ == "__main__":
    main()
