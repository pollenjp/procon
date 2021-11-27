import bisect
import collections
import logging
from logging import getLogger
from typing import List, Optional, Tuple

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)

# CONST_INT: int = 10 ** 18 + 1
CONST_INT: int = -1
CONST_N: int = 2 ** 20


def main():
    input_s: str = input().rstrip()
    input_k: int = int(input().rstrip())

    dot_x_list: List[Tuple[int, int]] = []
    current_list: List[int] = [None, None]
    pre_str: str = "X"
    cnt: int = 0
    for i, val in enumerate(input_s):
        if val == pre_str:
            cnt += 1
            continue

        if val == "X":
            current_list[0] = cnt
            if i == len(input_s) - 1:
                dot_x_list.append(tuple(current_list))
        else:  # "."
            current_list[1] = cnt
            dot_x_list.append(tuple(current_list))

        cnt = 1
        pre_str = val

    if input_s[-1] == "X":
        current_list[1] = cnt
        dot_x_list.append(tuple(current_list))
    else:
        current_list[0] = cnt
        current_list[1] = 0
        dot_x_list.append(tuple(current_list))
    logger.info(f"dot_x_list: {dot_x_list}")

    idx_start: int = 0
    cnt_k: int = 0
    cnt_x: int = dot_x_list[0][1]
    max_x: int = cnt_x
    flag: bool = True
    for i, (dot_num, x_num) in enumerate(dot_x_list[1:]):
        logger.info(f"{(dot_num, x_num)=}, {(cnt_k, cnt_x, max_x)=} {flag=}")
        current_idx = i + 1

        if dot_num > input_k:
            flag = False
            cnt_k = input_k
            cnt_x = x_num + input_k
            if max_x <= cnt_x:
                max_x = cnt_x
                flag = True
            idx_start = current_idx
            continue

        while (input_k - cnt_k) < dot_num and dot_num <= input_k:
            logger.info("while")
            cnt_k -= dot_x_list[idx_start][0]
            cnt_x -= dot_x_list[idx_start - 1][1]

            idx_start += 1
            flag = False

        cnt_k += dot_num
        cnt_x += x_num + dot_num
        if flag:
            max_x = cnt_x
        elif max_x <= cnt_x:
            max_x = cnt_x
            flag = True
    print(max_x)


if __name__ == "__main__":
    main()
