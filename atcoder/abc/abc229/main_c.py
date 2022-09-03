import logging
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def calc_area(a: int, b: int) -> int:
    return 4 * a * b + 3 * a + 3 * b


def main():
    input_n, input_w = map(int, input().rstrip().split())

    input_aw_list = [None] * input_n
    for i in range(input_n):
        input_aw_list[i] = list(map(int, input().rstrip().split()))

    # input_aw_list.sort(key=lambda x: x[1])
    input_aw_list.sort(key=lambda x: x[0], reverse=True)  # 美味しい順
    a_accum: int = 0
    w_accum: int = 0
    for a, w in input_aw_list:
        tmp_w = w_accum + w
        if tmp_w > input_w:
            a_accum += (input_w - w_accum) * a
            break
        a_accum += a * w
        w_accum = tmp_w

    print(f"{a_accum}")


if __name__ == "__main__":
    main()
