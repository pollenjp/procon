import logging
from logging import getLogger
from typing import List

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def calc_area(a: int, b: int) -> int:
    return 4 * a * b + 3 * a + 3 * b


def main():
    input_n, input_x = map(int, input().rstrip().split())
    input_a_list: List[int] = list(map(int, input().rstrip().split()))

    known_list: List[int] = [0] * len(input_a_list)

    p = input_x
    known_list[p - 1] = 1
    while known_list[input_a_list[p - 1] - 1] == 0:
        known_list[input_a_list[p - 1] - 1] = 1
        p = input_a_list[p - 1]

    print(f"{sum(known_list)}")


if __name__ == "__main__":
    main()
