import logging
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_d: int = int(input().rstrip())

    pressure: float = input_d / 100

    print(f"{pressure:.10f}")


if __name__ == "__main__":
    main()
