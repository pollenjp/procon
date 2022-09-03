import logging
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_t: int = int(input().rstrip())
    for i in range(input_t):
        input_a, input_s = map(int, input().rstrip().split())
        # a_bin, s_bin = bin(input_a), bin(input_s)
        a_bin = bin(input_a)
        x = input_s - 2 * input_a
        if x < 0:
            print("No")
            continue
        x_bin = bin(x)

        # TODO: check
        # max_bin_digit = max(len(a_bin), len(s_bin), len(x_bin)) - 2
        # logger.info(f"{a_bin[2:].zfill(max_bin_digit)=}")
        # logger.info(f"{s_bin[2:].zfill(max_bin_digit)=}")
        # logger.info(f"{x_bin[2:].zfill(max_bin_digit)=}")
        # logger.info(f"{int(a_bin, 2) & int(x_bin, 2)=}")
        if int(a_bin, 2) & int(x_bin, 2) == 0:
            print("Yes")
            continue
        print("No")


if __name__ == "__main__":
    main()
