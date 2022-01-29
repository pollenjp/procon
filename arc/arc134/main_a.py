import logging
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_n, input_l, input_w = map(int, input().rstrip().split())
    a_list = list(map(int, input().rstrip().split()))

    num_sheet: int = 0
    wrapped_end: int = 0
    x_wrap: int
    for a in a_list:
        not_wrapped_length = a - wrapped_end
        x_wrap = max(0, not_wrapped_length // input_w + (0 if not_wrapped_length % input_w == 0 else 1))
        num_sheet += x_wrap
        wrapped_end = max(a + input_w, wrapped_end + x_wrap * input_w)

    # last
    not_wrapped_length = input_l - wrapped_end
    x_wrap = max(0, not_wrapped_length // input_w + (0 if not_wrapped_length % input_w == 0 else 1))
    num_sheet += x_wrap

    print(f"{num_sheet}")


if __name__ == "__main__":
    main()
