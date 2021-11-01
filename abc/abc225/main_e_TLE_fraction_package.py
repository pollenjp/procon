import logging
from fractions import Fraction
from logging import getLogger
from typing import List, NewType, Tuple, Union

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)

INF = 10 ** 9 + 7

FractionType = NewType("FractionType", Union[Fraction, int, float])


def main():

    input_n: int = int(input().rstrip())

    xy_cooridnates: List[Tuple[int, int]] = []

    for _ in range(input_n):
        xy_cooridnates.append(tuple(map(int, input().rstrip().split(" "))))

    # logger.info(xy_cooridnates)

    # (start, end)
    tangent_list: List[Tuple[FractionType, FractionType]] = []
    for x, y in xy_cooridnates:
        start = Fraction(y - 1, x)
        if x - 1 > 0:
            end = Fraction(y, x - 1)
        else:
            end = Fraction(INF)
        tangent_list.append((start, end))

    # 終了位置でソート
    tangent_list_sorted = sorted(tangent_list, key=lambda x: x[1])
    # logger.info(tangent_list_sorted)

    cnt: int = 0
    current_tangent: FractionType = 0
    for start, end in tangent_list_sorted:
        if current_tangent <= start:
            cnt += 1
            current_tangent = end

    print(cnt)


if __name__ == "__main__":
    main()
