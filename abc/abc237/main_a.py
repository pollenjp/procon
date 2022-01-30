import logging
from dataclasses import dataclass
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_n: int = int(input().rstrip())
    a = 2 ** 31
    if (input_n >= -a) and (input_n < a):
        print("Yes")
        return
    print("No")


if __name__ == "__main__":
    main()
