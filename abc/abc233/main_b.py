import logging
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_l, input_r = map(int, input().rstrip().split())
    input_s = input().rstrip()
    char_list = [c for c in input_s]

    tmp_char_list = []
    for i in range(input_r - 1, input_l - 1 - 1, -1):
        tmp_char_list.append(char_list[i])

    for i, j in enumerate(range(input_l - 1, input_r)):
        char_list[j] = tmp_char_list[i]

    print(f"{''.join(char_list)}")


if __name__ == "__main__":
    main()
