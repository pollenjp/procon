import logging
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_s, input_t, input_x = map(int, input().rstrip().split())

    t = input_t
    if t < input_s:
        t += 24
    x = input_x
    if x < input_s:
        x += 24

    if input_s <= x < t:
        print("Yes")
        return
    print("No")


if __name__ == "__main__":
    main()
