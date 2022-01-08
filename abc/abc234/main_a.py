import logging
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_x, input_y = map(int, input().rstrip().split())

    left = input_y - input_x

    ans = 0
    if left > 0:
        ans = (left // 10) + (0 if left % 10 == 0 else 1)
    print(f"{ans}")


if __name__ == "__main__":
    main()
