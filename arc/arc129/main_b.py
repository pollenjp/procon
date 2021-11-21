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
    input_n: int = int(input())

    l_max: int = 0
    r_min: int = 10 ** 9 + 1

    for _ in range(input_n):
        input_l, input_r = map(int, input().rstrip().split())

        l_max = max(l_max, input_l)
        r_min = min(r_min, input_r)

        if l_max <= r_min:
            print("0")
            continue

        center: int = (l_max + r_min) // 2
        print(f"{l_max - center}")


if __name__ == "__main__":
    main()
