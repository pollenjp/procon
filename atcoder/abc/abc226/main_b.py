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
    input_n: int = int(input())

    l_str_list: List[str] = [""] * input_n
    for i in range(input_n):
        l_str_list[i] = input()

    print(len(set(l_str_list)))


if __name__ == "__main__":
    main()
