import logging
from logging import getLogger
from typing import List

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)

CONST_VAL: int = 998244353


def main():
    input_n: int = int(input().rstrip())
    input_a_list: List[int] = list(map(int, input().rstrip().split()))

    a_stack: List[int] = input_a_list[::-1]

    k_to_cnt_current = [0 for _ in range(10)]
    k_to_cnt_next = [0 for _ in range(10)]
    k_to_cnt_current[a_stack.pop(-1)] += 1

    for i_th in range(input_n - 1):
        y: int = a_stack.pop(-1)
        for k0, cnt_k0 in enumerate(k_to_cnt_current):
            x: int = k0
            k1: int = (x + y) % 10
            k_to_cnt_next[k1] += cnt_k0
            k_to_cnt_next[k1] %= CONST_VAL
            k1: int = (x * y) % 10
            k_to_cnt_next[k1] += cnt_k0
            k_to_cnt_next[k1] %= CONST_VAL

        k_to_cnt_current = k_to_cnt_next
        k_to_cnt_next = [0 for _ in range(10)]

    for k, cnt in enumerate(k_to_cnt_current):
        print(cnt)


if __name__ == "__main__":
    main()
