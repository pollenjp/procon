import logging
from logging import getLogger
from typing import Dict, List

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_s_list: List[str] = list(map(str, input().rstrip().split()))
    input_t_list: List[str] = list(map(str, input().rstrip().split()))

    s_list = input_s_list.copy()
    s_idxes: Dict[str, int] = {char: i for i, char in enumerate(input_s_list)}

    num_trial: int = 0

    for t_idx, t in enumerate(input_t_list):
        s_idx: int = s_idxes[t]
        if s_idx != t_idx:
            tmp = s_list[t_idx]
            s_list[t_idx] = t
            s_list[s_idx] = tmp
            s_idxes[t] = t_idx
            s_idxes[tmp] = s_idx
            num_trial += 1
    if num_trial % 2 == 1:
        print("No")
        return
    print("Yes")


if __name__ == "__main__":
    main()
