import logging
import math
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_a, input_b = map(int, input().rstrip().split())

    if input_b - input_a == 1:
        print("Yes")
        return
    if input_b == 10 and input_a == 1:
        print("Yes")
        return
    print("No")


if __name__ == "__main__":
    main()
