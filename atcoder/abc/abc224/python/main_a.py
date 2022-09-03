import logging
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_s: str = input().rstrip()

    if input_s[-1] == "r":
        print("er")
    else:
        print("ist")


if __name__ == "__main__":
    main()
