import logging
from logging import getLogger
from typing import List

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def include_leading_zero(input_str: str) -> bool:
    if input_str[0] == "0":
        return True

    return False


def main():
    input_n: int = int(input().rstrip())

    # num_char_list: List[str] = [s for s in str(input_n)]
    num_char_list: List[str] = sorted(str(input_n), reverse=True)

    num_a_list: List[str] = []
    num_b_list: List[str] = []
    ab_list: List[List[str]] = [num_a_list, num_b_list]

    ab_pointer: int = 0
    for char_idx, num_char in enumerate(num_char_list):
        logger.info(f"{num_char=}")
        ab_list[ab_pointer].append(num_char)

        num_a_list, num_b_list = ab_list

        if len(num_a_list) < 1 or len(num_b_list) < 1:
            ab_pointer = (ab_pointer + 1) % 2
            continue

        num_a_str = "".join(num_a_list)
        num_b_str = "".join(num_b_list)
        if int(num_a_str) >= int(num_b_str):
            ab_pointer = 1
        else:
            ab_pointer = 0

    num_a_list, num_b_list = ab_list
    logger.info(f"{num_a_list=}, {num_b_list=}")
    num_a_str = "".join(num_a_list)
    num_b_str = "".join(num_b_list)
    if include_leading_zero(num_b_str) or include_leading_zero(num_b_str):
        raise Exception("invalid input")

    ans: int = int(num_a_str) * int(num_b_str)
    print(f"{ans}")


if __name__ == "__main__":
    main()
