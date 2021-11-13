import bisect
import logging
from logging import getLogger
from typing import List

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():
    input_n, input_k = map(int, input().rstrip().split())
    a_list: List[int] = list(map(int, input().rstrip().split()))

    a_list.sort()
    cnt: int = 0

    while len(a_list) >= input_k:
        cnt += 1
        a_list, tmp_list = a_list[:-input_k], a_list[-input_k:]
        assert len(tmp_list) == input_k
        for val in tmp_list:
            val -= 1
            if val != 0:
                bisect.insort_left(a_list, val)

    print(cnt)


if __name__ == "__main__":
    main()
