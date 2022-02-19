import logging
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)

const_angle_mod = 360


def main():

    input_x = int(input().rstrip())
    minus: bool = False if input_x >= 0 else True
    x_str: str = str(abs(input_x))
    digit_1 = int(x_str[-1])
    ans = int(x_str[:-1]) if x_str[:-1] else 0
    if minus:
        if digit_1 > 0:
            ans += 1
        print(f"{-ans}")
    else:
        print(f"{ans}")

    # print(f"{math.floor(input_x / 10)}")


if __name__ == "__main__":
    main()
