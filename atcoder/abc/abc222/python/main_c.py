import logging
from logging import getLogger
from typing import Dict, List

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)

GCP_DICT: Dict[str, int] = {
    "G": 0,
    "C": 1,
    "P": 2,
}


def gcp(one: str, other: str):
    one_num: int = GCP_DICT[one]
    other_num: int = GCP_DICT[other]

    if one_num == other_num:
        return 0  # あいこ

    if (one_num + 1) % len(GCP_DICT) == other_num:
        return 1  # one の勝ち

    if (one_num - 1) % len(GCP_DICT) == other_num:
        return -1  # one の負け

    raise Exception(f"Not supported!: {one=}, {other=}")


def main():

    input_n, input_m = list(map(int, input().rstrip().split(" ")))
    a_str_list: List[int] = []
    for i in range(input_n * 2):
        a_str_list.append(input().rstrip())

    ranking_tuple_list: List[List[int, int]] = [[i, 0] for i, _ in enumerate(a_str_list)]

    for m_idx in range(input_m):
        for k in range(0, len(a_str_list), 2):
            one: List[int, int] = ranking_tuple_list[k]
            other: List[int, int] = ranking_tuple_list[k + 1]

            one_gcp: str = a_str_list[one[0]][m_idx]
            other_gcp: str = a_str_list[other[0]][m_idx]

            gcp_result: int = gcp(one_gcp, other_gcp)

            if gcp_result > 0:
                one[1] += 1
            elif gcp_result < 0:
                other[1] += 1
            # logger.info(f"{one_gcp=}, {other_gcp=}, {gcp_result=}")
        # logger.info(f"{ranking_tuple_list=}")
        ranking_tuple_list = sorted(ranking_tuple_list, key=lambda x: (-x[1], x[0]))

    for (i, _) in ranking_tuple_list:
        print(f"{(i+1)}")


if __name__ == "__main__":
    main()
