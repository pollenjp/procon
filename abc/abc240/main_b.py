import logging
from logging import getLogger
from typing import List, Set

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_n = int(input().rstrip())
    a_list: List[int] = list(map(int, input().rstrip().split()))
    a_set: Set[int] = set(a_list)
    print(f"{len(a_set)}")


if __name__ == "__main__":
    main()
