import logging
from logging import getLogger
from typing import Dict, List, Tuple

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_n: int = int(input().rstrip())
    len_to_idx_list: Dict[int, List[int]] = {}
    l_tuple_list: List[Tuple] = [()] * input_n
    l_tuple_idx: int = 0
    for _ in range(input_n):
        l_list: List[int] = list(map(int, input().rstrip().split()))
        num_l: int = l_list[0]
        l_tuple: Tuple = tuple(l_list[1:])

        duplicate: bool = False
        if num_l in len_to_idx_list:
            for i in len_to_idx_list[num_l]:
                if l_tuple_list[i] == l_tuple:
                    duplicate = True
                    break

        if not duplicate:
            if num_l not in len_to_idx_list:
                len_to_idx_list[num_l] = []
            l_tuple_list[l_tuple_idx] = l_tuple
            l_tuple_idx += 1
            len_to_idx_list[num_l].append(l_tuple_idx)

    print(l_tuple_idx)


if __name__ == "__main__":
    main()
