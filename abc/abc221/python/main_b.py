import logging
from logging import getLogger
from typing import List

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_s: str = input().rstrip()
    input_t: str = input().rstrip()

    new_t_list: List[str] = [s for s in input_t]

    for char_idx, (s_char, t_char) in enumerate(zip(input_s, input_t)):
        if s_char != t_char and char_idx < len(input_s) - 1:
            tmp = new_t_list[char_idx]
            new_t_list[char_idx] = new_t_list[char_idx + 1]
            new_t_list[char_idx + 1] = tmp
            break
    new_t: str = "".join(new_t_list)
    logger.info(f"{new_t=}")
    if input_s == new_t:
        print("Yes")
        return
    print("No")


if __name__ == "__main__":
    main()
