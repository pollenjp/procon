import logging
from logging import getLogger
from typing import List

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_k: int = int(input().rstrip())

    if input_k == 1:
        print(2)
        return

    # check num of digits
    num_of_digits: int = 1
    x = 1
    while True:
        # logger.info(f"{x=}")
        # logger.info(f"{num_of_digits=}")
        if x <= input_k and input_k < x * 2:
            break
        x *= 2
        num_of_digits += 1

    # logger.info(f"{num_of_digits=}")

    # detect 何番目か
    left_k: int = input_k - x
    left_k_bin: str = bin(left_k)[2:]
    # logger.info(f"{left_k_bin=}")
    # logger.info(f"{left_k_bin.zfill(num_of_digits-1)=}")

    ans_list: List[str] = ["0"] * num_of_digits
    ans_list[0] = "2"
    for i, val in enumerate(left_k_bin.zfill(num_of_digits - 1)):
        if val == "1":
            ans_list[i + 1] = "2"

    ans: str = "".join(ans_list)
    print(f"{ans}")


if __name__ == "__main__":
    main()
