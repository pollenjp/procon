import logging
from logging import getLogger
from typing import List

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def calc_area(a: int, b: int) -> int:
    return 4 * a * b + 3 * a + 3 * b


def main():
    input_n: int = int(input())
    input_s_list: List[int] = list(map(int, input().rstrip().split()))

    input_s_list.sort()

    # logger.info(f"{input_s_list=}")

    max_s: int = max(input_s_list)
    # logger.info(f"{max_s=}")

    cnt: int = 0
    for s_i in input_s_list:
        a, b = 1, 1
        s_i_flag: bool = False
        for a in range(1, s_i + 1):
            for b in range(1, s_i + 1):
                if s_i == calc_area(a, b):
                    s_i_flag = True
                    break
            if s_i_flag:
                break
        if not s_i_flag:
            cnt += 1

    print(cnt)


if __name__ == "__main__":
    main()
