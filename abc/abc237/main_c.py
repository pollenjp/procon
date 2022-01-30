import logging
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)

const_c: str = "a"


def main():

    input_s = input().rstrip()

    left_idx: int = 0
    right_idx: int = 0

    for i, c in enumerate(input_s):
        if c != const_c:
            left_idx = i
            break

    for i, c in enumerate(reversed(input_s)):
        if c != const_c:
            right_idx = len(input_s) - i - 1
            break

    if left_idx > len(input_s) - (right_idx + 1):
        print("No")
        return

    sub_s: str = input_s[left_idx : right_idx + 1]

    for i in range(len(sub_s) // 2):
        # logger.info(f"{sub_s[i]=}, {sub_s[-(i + 1)]=}")
        if sub_s[i] != sub_s[-(i + 1)]:
            print("No")
            return

    print("Yes")


if __name__ == "__main__":
    main()
