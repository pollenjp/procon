import logging
from logging import getLogger
from typing import Dict

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_n: int = int(input().rstrip())

    cnt_dict: Dict["str", int] = {}

    for i in range(input_n):
        s_i: str = input().rstrip()
        if s_i in cnt_dict:
            cnt_dict[s_i] += 1
        else:
            cnt_dict[s_i] = 1

    name_max: str = ""
    cnt_max: int = 0
    for name, cnt in cnt_dict.items():
        if cnt >= cnt_max:
            cnt_max = cnt
            name_max = name

    print(f"{name_max}")


if __name__ == "__main__":
    main()
