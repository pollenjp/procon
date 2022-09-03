import logging
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_n, input_l, input_r = map(int, input().rstrip().split())

    n_bin_str: str = bin(input_n)[2:]
    # logger.info(f"{n_bin_str=}")
    # logger.info(f"{type(n_bin_str)=}")

    cnt: int = 0
    for i, ith_bin_val in enumerate(n_bin_str[::-1]):
        # i + 1 桁目のビット値
        # logger.info(f"{i=}, {ith_bin_val=}")
        if ith_bin_val == "0":
            continue

        # TODO:
        x_min: int = 2 ** i
        x_max: int = 2 ** (i + 1) - 1
        cnt_valid: int = min(input_r, x_max) - max(input_l, x_min) + 1
        if cnt_valid > 0:
            cnt += cnt_valid
        # logger.info(f"{cnt_valid=}, {cnt=}")

    print(cnt)


if __name__ == "__main__":
    main()
