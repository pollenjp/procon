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
    input_a_list = list(map(int, input().rstrip().split()))
    cnt_list = [0] * (input_n)
    for val in input_a_list:
        cnt_list[val - 1] += 1

    for i, cnt in enumerate(cnt_list):
        if cnt == 3:
            print(f"{i + 1}")
            break


if __name__ == "__main__":
    main()
