import logging
from logging import getLogger
from typing import List, Tuple

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)

INF = 10 ** 9 + 7


class Fraction:
    def __init__(self, numerator: int, denominator: int = 1):
        if not isinstance(numerator, int):
            raise TypeError("numerator must be int")
        if not isinstance(denominator, int):
            raise TypeError("denominator must be int")
        self.numerator: int = numerator  # 分子
        self.denominator: int = denominator  # 分母

    def __lt__(self, other):
        return self.numerator * other.denominator < self.denominator * other.numerator

    def __le__(self, other):
        return self.numerator * other.denominator <= self.denominator * other.numerator


def main():

    input_n: int = int(input().rstrip())

    xy_cooridnates: List[Tuple[int, int]] = []

    for _ in range(input_n):
        xy_cooridnates.append(tuple(map(int, input().rstrip().split(" "))))

    # logger.info(xy_cooridnates)

    # (start, end)
    tangent_list: List[Tuple[int, int]] = []
    for x, y in xy_cooridnates:
        start: Fraction = Fraction(y - 1, x)
        if x - 1 > 0:
            end: Fraction = Fraction(y, x - 1)
        else:
            end: Fraction = Fraction(INF, 1)
        tangent_list.append((start, end))

    # 終了位置でソート
    tangent_list_sorted = sorted(tangent_list, key=lambda x: x[1])
    # logger.info(tangent_list_sorted)

    cnt: int = 0
    current_tangent: Fraction = Fraction(0)
    for start, end in tangent_list_sorted:
        if current_tangent <= start:
            cnt += 1
            current_tangent = end

    print(cnt)


if __name__ == "__main__":
    main()
