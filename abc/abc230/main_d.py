import logging
from logging import getLogger
from operator import itemgetter
from typing import List, Tuple

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():
    input_n, input_d = map(int, input().split())

    lr_list: List[Tuple[int, int]] = [0] * input_n
    for i in range(input_n):
        lr_list[i] = tuple(map(int, input().split()))

    # 区間の終端でソート
    lr_list_sorted = sorted(lr_list, key=itemgetter(1))

    # 前回除いた橋
    removed = -1
    ans = 0
    for l, r in lr_list_sorted:
        if l > removed:
            removed = r + (input_d - 1)
            ans += 1

    print(ans)


if __name__ == "__main__":
    main()
