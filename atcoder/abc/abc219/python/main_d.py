import logging
import pprint
from logging import getLogger
from typing import List

import numpy as np

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)

CONST_INF: int = 400


def main():
    input_n: int = int(input().rstrip())
    x, y = list(map(int, input().rstrip().split(" ")))

    ab_list: List[int] = []
    for i in range(input_n):
        ab_list.append(list(map(int, input().rstrip().split(" "))))

    dp_list: List[np.ndarray]
    # dp_list = [np.full((x + 1, y + 1), fill_value=CONST_INF, dtype=np.int32) for _ in range(input_n + 1)]
    dp_list = [np.full((x + 1, y + 1), fill_value=CONST_INF, dtype=np.int32) for _ in range(input_n + 1)]
    dp_list[0][0, 0] = 0

    for i_0org, ab in enumerate(ab_list):
        a, b = ab
        prev_ndarray = dp_list[i_0org]
        current_ndarray = dp_list[i_0org + 1]

        if a <= 0 or b <= 0:
            raise Exception()

        # はみ出さないやつ
        if a <= x and b <= y:
            current_ndarray[a:, b:] = np.minimum(current_ndarray[a:, b:], prev_ndarray[: x - a + 1, : y - b + 1] + 1)
        if 0 < a and a <= x:
            # 右にだけはみ出すやつ
            current_ndarray[a:, -1] = np.minimum(
                current_ndarray[a:, -1], prev_ndarray[: x - a + 1, max(0, y - b + 1) :].min(axis=1) + 1
            )
        if 0 < b and b <= y:
            # 下にだけはみ出すやつ
            current_ndarray[-1, b:] = np.minimum(
                current_ndarray[-1, b:], prev_ndarray[max(0, x - a + 1) :, : y - b + 1].min(axis=0) + 1
            )
        if a > 0 and b > 0:
            # 右下にはみ出すやつ
            current_ndarray[-1, -1] = np.minimum(
                current_ndarray[-1, -1], prev_ndarray[max(0, x - a + 1) :, max(0, y - b + 1) :].min(axis=None) + 1
            )

        current_ndarray[...] = np.minimum(current_ndarray, prev_ndarray)

    logger.info(f"dp_list=\n{pprint.pformat(dp_list)}")

    if dp_list[-1][-1, -1] == CONST_INF:
        print(-1)
        return

    print(dp_list[-1][-1, -1])


if __name__ == "__main__":
    main()
