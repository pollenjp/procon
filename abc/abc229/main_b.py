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
    input_a, input_b = map(int, input().rstrip().split())

    for (a, b) in zip(str(input_a)[::-1], str(input_b)[::-1]):
        if int(a) + int(b) >= 10:
            print("Hard")
            return
    print("Easy")


if __name__ == "__main__":
    main()
