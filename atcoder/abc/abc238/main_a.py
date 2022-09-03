import logging
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_n: int = int(input().rstrip())
    if input_n >= 5:
        print("Yes")
        return
    if 2 ** input_n > input_n ** 2:
        print("Yes")
        return
    print("No")


if __name__ == "__main__":
    main()
