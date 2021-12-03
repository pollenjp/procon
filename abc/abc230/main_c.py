import logging
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def is_draw(row, column, a, b):
    if (row - a) + (column - b) == 0 or (row - a) - (column - b) == 0:
        return True
    return False


def main():
    input_n, input_a, input_b = map(int, input().rstrip().split())
    input_p, input_q, input_r, input_s = map(int, input().rstrip().split())

    ans_row_num: int = input_q + 1 - input_p
    ans_column_num: int = input_s + 1 - input_r

    ans_list = [0] * ans_row_num
    for j in range(ans_row_num):
        ans_list[j] = ["."] * ans_column_num

    logger.info(f"{ans_list}")

    for row in range(input_p, input_q + 1):
        for column in range(input_r, input_s + 1):
            # logger.info(f"{(row, column, input_a, input_b)=}")
            # logger.info(f"{is_draw(row, column, input_a, input_b)=}")
            if is_draw(row, column, input_a, input_b):
                ans_list[row - input_p][column - input_r] = "#"
        print(f"{''.join(ans_list[row - input_p])}")


if __name__ == "__main__":
    main()
