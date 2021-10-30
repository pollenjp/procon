import logging
from logging import getLogger
from typing import List

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():
    input_n: int = int(input().rstrip())
    del input_n
    input_a_list: List[int] = list(map(int, input().rstrip().split(" ")))
    input_x: int = int(input().rstrip())

    sum_of_a_list: int = sum(input_a_list)

    prod: int = input_x // sum_of_a_list
    cnt: int = prod * len(input_a_list)
    left: int = input_x - prod * sum_of_a_list
    logger.info(f"{cnt=}, {left=}")
    for a in input_a_list:
        logger.info(f"{cnt=}, {left=}, {a=}")
        if left < 0:
            break
        left -= a
        cnt += 1

    print(cnt)


if __name__ == "__main__":
    main()
