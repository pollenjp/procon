import logging
import sys
from logging import getLogger
from typing import List, Tuple

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
# logger.setLevel(logging.INFO)
logger.setLevel(logging.WARNING)  # 提出用


def get_conditional_sum(
    a_list: List[int],
    mapping_list: List[Tuple[int, int]] = None,  # const
) -> Tuple[int, int]:

    num_cases: int
    conditional_sum: int

    if len(a_list) >= 3:

        logger.info(a_list)
        b_list: List[int] = a_list[1:]
        b_num_cases, b_conditional_sum = mapping_list[len(b_list) - 1]

        c_list: List[int] = a_list[2:]
        c_num_cases, c_conditional_sum = mapping_list[len(c_list) - 1]

        num_cases: int = b_num_cases + c_num_cases
        conditional_sum: int = (
            b_num_cases * a_list[0] + b_conditional_sum + c_num_cases * (a_list[0] - a_list[1]) + c_conditional_sum
        )

    elif len(a_list) == 2:
        num_cases: int = 2
        conditional_sum: int = 2 * a_list[0]
    elif len(a_list) == 1:
        num_cases: int = 1
        conditional_sum: int = a_list[0]
    else:
        raise ValueError(f"Not supported length! {len(a_list)}")

    logger.info(f"(num_cases, conditional_sum) = ({num_cases}, {conditional_sum})")
    return num_cases, conditional_sum


def solve(n: int, a_list: List[int]) -> int:

    # 条件を満たす総和を求める

    # mapping table を作る
    # init mapping list
    mapping_list = [None for i in range(len(a_list))]
    logger.info(a_list)
    for i in range(1, len(a_list) + 1):
        logger.info(f"i = {i}")
        tmp_list: List[int] = a_list[-i::]
        logger.info(f"tmp_list = {tmp_list}")
        logger.info(mapping_list)
        mapping_list[len(tmp_list) - 1] = get_conditional_sum(
            a_list=tmp_list,
            mapping_list=mapping_list,  # const
        )
        logger.info(f"after mapping_list = {mapping_list}")

    conditional_sum: int
    _, conditional_sum = mapping_list[len(a_list) - 1]

    # 総和を 10^9 + 7 で割った余り
    return conditional_sum % (10 ** 9 + 7)


if __name__ == "__main__":
    N = map(int, sys.stdin.readline().rstrip())
    a_list: List[int] = list(map(int, sys.stdin.readline().rstrip().split(" ")))
    print(solve(N, a_list))
