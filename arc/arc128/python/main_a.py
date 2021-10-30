import logging
from logging import getLogger
from typing import List, Optional

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)

CONST_MAX: int = 1e9 + 1


def main():
    input_n: int = int(input())
    input_a_list: List[float] = list(map(float, input().split()))

    # G -> S or S -> G の交換を行うときの A_i
    gold_to_silver_idx: int = 0
    silver_to_gold_idx: int = 0
    silver_to_gold_a: Optional[float] = None

    exchange_list: List[int] = [0 for _ in range(input_n)]

    # 単調減少部分列を抽出
    for a_idx, a_i in enumerate(input_a_list + [CONST_MAX]):
        # logger.info(f"{a_idx}, {a_i}, {exchange_list=}, {silver_to_gold_idx=}, {gold_to_silver_idx=}")

        if a_idx == 0:
            # logger.info("first")
            # 第1要素に対する処理
            gold_to_silver_idx = a_idx
            silver_to_gold_idx = a_idx
            silver_to_gold_a = a_i
            continue

        if a_i < silver_to_gold_a:
            # logger.info("update gold_to_silver_a")
            # 候補
            silver_to_gold_idx = a_idx
            silver_to_gold_a = a_i
            continue

        if a_i > silver_to_gold_a:
            # logger.info("exchange check")
            if gold_to_silver_idx == silver_to_gold_idx:
                # 再び初期化
                gold_to_silver_idx = a_idx
                silver_to_gold_idx = a_idx
                silver_to_gold_a = a_i
                continue

            # G -> S, S -> G の交換を一気に行う
            exchange_list[gold_to_silver_idx] = 1
            exchange_list[silver_to_gold_idx] = 1

            # 再び初期化
            gold_to_silver_idx = a_idx
            silver_to_gold_idx = a_idx
            silver_to_gold_a = a_i

    # logger.info(f"final: {exchange_list=}")
    out_str: str = " ".join(list(map(str, exchange_list)))
    print(out_str)


if __name__ == "__main__":
    main()
