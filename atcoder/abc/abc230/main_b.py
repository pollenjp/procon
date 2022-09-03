import logging
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():
    input_s: str = input().rstrip()

    if len(input_s) == 1:
        print("Yes")
        return

    loop_three_str: str = "oxx"

    x: int = 0
    if input_s[0] == "o":
        x = 0
    elif input_s[:2] == "xx":
        x = 1
    elif input_s[:2] == "xo":
        x = 2
    else:
        print("No")
        return

    flag: bool = True
    for i in range(0, len(input_s)):
        if input_s[i] != loop_three_str[(x + i) % len(loop_three_str)]:
            flag = False
            break

    if flag:
        print("Yes")
    else:
        print("No")


if __name__ == "__main__":
    main()
