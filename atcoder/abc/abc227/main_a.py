import logging
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_n, input_k, input_a = map(int, input().rstrip().split())

    a_list = [i for i in range(input_a, input_n + 1)] + [i for i in range(1, input_a)]
    # logger.info(f"{a_list=}")
    print(a_list[(input_k - 1) % input_n])


if __name__ == "__main__":
    main()
