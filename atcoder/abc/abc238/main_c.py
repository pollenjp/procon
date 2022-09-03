import logging
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)

const_mod: int = 998244353


def main():

    input_n: int = int(input().rstrip())

    # g(n) = (nの桁数) - 1
    g_n: int = len(str(input_n)) - 1

    ans = 0
    first_a = 1
    n = input_n
    last_a = input_n
    ans += (n * (first_a + last_a)) // 2
    ans %= const_mod
    # logger.info(f"{ans=}")

    for i in range(1, g_n + 1):
        x1 = 10 ** i - 1
        x2 = min(10 ** (i + 1) - 1, input_n)
        # logger.info(f"{x1=}, {x2=}")
        ans -= (x1 % const_mod) * (x2 - x1)
        # logger.info(f"{ans=}")
        ans %= const_mod
        # logger.info(f"{ans=}")

    print(f"{ans}")


if __name__ == "__main__":
    main()
