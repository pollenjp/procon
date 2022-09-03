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

    a: int = int(input_s[0])
    b: int = int(input_s[2])

    print(f"{a * b}")


if __name__ == "__main__":
    main()
