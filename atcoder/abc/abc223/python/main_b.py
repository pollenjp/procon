import logging
from logging import getLogger
from typing import List

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def solve(x: int):

    if x == 0:
        return False

    if x % 100 == 0:
        return True

    return False


def main():

    input_s: str = input().rstrip()

    string_list: List[str] = []
    for i in range(len(input_s)):
        string_list.append(input_s[i:] + input_s[:i])

    string_list.sort()

    print(string_list[0])
    print(string_list[-1])


if __name__ == "__main__":
    main()
