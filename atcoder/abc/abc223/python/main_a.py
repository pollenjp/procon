import logging
from logging import getLogger

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

    input_x: int = int(input())

    if solve(input_x):
        print("Yes")
    else:
        print("No")


if __name__ == "__main__":
    main()
