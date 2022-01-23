import itertools
import logging
from logging import getLogger
from typing import List, Tuple

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def bit(n, k):
    return (n >> k) & 1


def main():

    input_n = int(input().rstrip())
    a_list_list: List[List[int]] = []
    for _ in range(2 * input_n - 1):
        a_list_list.append(list(map(int, input().rstrip().split())))
    # logger.info(f"{a_list_list=}")

    def calc_reward(x: List[Tuple[int, int]]) -> int:
        b_ret = None
        for (i, j) in x:
            if i > j:
                i, j = j, i
            if b_ret is None:
                b_ret = a_list_list[i][j - i - 1]
            else:
                b_ret = b_ret ^ a_list_list[i][j - i - 1]
        return b_ret

    persons_list: List[int] = [i for i in range(2 * input_n)]
    persons_set = set(persons_list)

    # for i in persons_set:
    #     # logger.info(f"{i=}")

    ans: int = 0
    v: Tuple[...]
    for v in itertools.combinations(persons_set - set([0]), input_n - 1):
        v_left = set((0,) + v)
        v_right = persons_set - v_left

        # TODO: v_right : permutation
        for v2_right in itertools.permutations(v_right):
            skip_flag = False
            for v_l, v_r in zip(v_left, v2_right):
                if v_l > v_r:
                    skip_flag = True
                    break
            if skip_flag:
                continue
            # logger.info(f"{(v_left, v2_right)=}")

            ans = max(ans, calc_reward([(i, j) for i, j in zip(v_left, v2_right)]))

    print(f"{ans}")


if __name__ == "__main__":
    main()
