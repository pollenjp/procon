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
    input_a, input_b = map(int, input().rstrip().split())

    s_list = [c for c in input_s]
    tmp = s_list[input_a - 1]
    s_list[input_a - 1] = s_list[input_b - 1]
    s_list[input_b - 1] = tmp
    print(f"{''.join(s_list)}")


if __name__ == "__main__":
    main()
