import logging
import math
from logging import getLogger
from typing import List, Tuple

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)

INF = 10 ** 9 + 7


def main():

    input_n: int = int(input().rstrip())

    xy_cooridnates: List[Tuple[int, int]] = []

    for _ in range(input_n):
        xy_cooridnates.append(tuple(map(int, input().rstrip().split(" "))))

    logger.info(xy_cooridnates)

    # (start, end)
    tangent_list: List[Tuple[int, int]] = []
    for x, y in xy_cooridnates:
        start: float = (y - 1) / x
        if x - 1 > 0:
            end: float = y / (x - 1)
        else:
            end: float = INF
        tangent_list.append((start, end))

    # 終了位置でソート
    tangent_list_sorted = sorted(tangent_list, key=lambda x: x[1])
    logger.info(tangent_list_sorted)

    cnt: int = 0
    current_tangent: float = 0
    for start, end in tangent_list_sorted:
        if current_tangent <= start:
            cnt += 1
            current_tangent = end

    print(cnt)


if __name__ == "__main__":
    main()
