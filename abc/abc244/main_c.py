import logging
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_n: int = int(input().rstrip())
    use_idx: int = 1
    left = {i: True for i in range(1, input_n * 2 + 1 + 1)}

    # first
    while left[use_idx] is False:
        use_idx += 1
    print(use_idx, flush=True)
    left[use_idx] = False
    use_idx += 1

    while True:
        x = int(input().rstrip())
        if x == 0:
            return
        left[x] = False

        while left[use_idx] is False:
            use_idx += 1
        print(use_idx, flush=True)
        left[use_idx] = False
        use_idx += 1


if __name__ == "__main__":
    main()
