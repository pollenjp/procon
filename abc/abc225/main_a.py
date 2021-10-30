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

    char_set = set([c for c in input_s])

    if len(char_set) == 3:
        print("6")
    elif len(char_set) == 2:
        print("3")
    else:
        print("1")


if __name__ == "__main__":
    main()
