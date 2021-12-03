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

    if input_n >= 42:
        print(f"AGC{str(input_n + 1).zfill(3)}")
        return
    print(f"AGC{str(input_n).zfill(3)}")


if __name__ == "__main__":
    main()
