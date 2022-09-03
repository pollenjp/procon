import logging
from logging import getLogger
from typing import List, Tuple

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_n: int = int(input().rstrip())
    ab_tuple_list: List[Tuple[float, float]] = []
    for _ in range(input_n):
        ab_tuple_list.append(tuple(map(float, input().rstrip().split())))

    converted_length_list: List[float] = [a / b for (a, b) in ab_tuple_list]
    converted_length_all: float = sum(converted_length_list)
    logger.info(f"{converted_length_all=}")

    converted_length_accumlated: float = 0
    target_length_converted: float = converted_length_all / 2

    target_length: float = 0
    for i, ((a, b), converted_length) in enumerate(zip(ab_tuple_list, converted_length_list)):
        converted_length_accumlated += converted_length
        if converted_length_accumlated < target_length_converted:
            target_length += a
            continue
        target_length += (
            a * (target_length_converted - (converted_length_accumlated - converted_length)) / converted_length
        )
        break
    print(f"{target_length:.10}")


if __name__ == "__main__":
    main()
