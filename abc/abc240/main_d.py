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

    input_n = int(input().rstrip())

    a_list = list(map(int, input().rstrip().split()))

    stack: List[Tuple[int, int]] = []  # (ball number, continuous same ball cont)
    for a in a_list:
        if len(stack) == 0:
            stack.append((a, 1))
            print("1")
            continue

        pre_val, pre_cnt = stack[-1]
        if a == pre_val:
            if pre_cnt + 1 >= a:
                # TODO: more efficient way to do this
                for _ in range(pre_cnt):
                    stack.pop(-1)
            else:
                stack.append((a, pre_cnt + 1))
        else:
            stack.append((a, 1))
        # logger.info(f"{stack=}")
        print(f"{len(stack)}")


if __name__ == "__main__":
    main()
