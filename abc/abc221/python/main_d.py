import logging
from logging import getLogger
from typing import List, Tuple

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.ERROR)


def main():
    input_n: int = int(input().rstrip())
    input_a_list: List[int] = []
    input_b_list: List[int] = []
    for i in range(input_n):
        a, b = map(int, input().rstrip().split())
        input_a_list.append(a)
        input_b_list.append(b)

    date_in_out_list: List[Tuple[int, int]] = []
    date_in_out_list += [(a, 1) for a in input_a_list]
    date_in_out_list += [(a + b, 0) for (a, b) in zip(input_a_list, input_b_list)]

    date_in_out_list = sorted(
        date_in_out_list,
        key=lambda x: (x[0], x[1]),
    )
    # logger.info(f"{date_in_out_list=}")

    cnt_list: List[int] = [0 for i in range(input_n)]
    num_joined_player: int = 0
    date_idx: int
    current_date: int
    is_in: bool
    pre_date: int = input_a_list[0]
    for date_idx, (current_date, is_in) in enumerate(date_in_out_list):
        # logger.info(f"{cnt_list=}")
        if num_joined_player > 0:
            cnt_list[num_joined_player - 1] += current_date - pre_date
        if is_in == 1:
            num_joined_player += 1
        else:
            num_joined_player -= 1

        pre_date = current_date

    # logger.info(f"{cnt_list=}")
    for cnt in cnt_list:
        print(f"{cnt}", end=" ")


if __name__ == "__main__":
    main()
