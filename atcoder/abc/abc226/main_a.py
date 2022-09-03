import logging
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_x: float = float(input().rstrip())

    if input_x >= int(input_x) + 0.5:
        print(f"{int(input_x) + 1}")
    else:
        print(f"{int(input_x)}")


if __name__ == "__main__":
    main()
