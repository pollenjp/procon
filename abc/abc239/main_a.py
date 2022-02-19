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

    input_h: int = int(input().rstrip())
    print(f"{math.sqrt(input_h * (12800000 + input_h))}")


if __name__ == "__main__":
    main()
