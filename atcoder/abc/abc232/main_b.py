import logging
from logging import getLogger
from typing import Dict, List

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_s: str = input().rstrip()
    input_t: str = input().rstrip()

    alphabet: str = "abcdefghijklmnopqrstuvwxyz"
    alphabet_dict: Dict[str, int] = {alphabet[i]: i for i in range(len(alphabet))}

    # 先頭文字を何文字ずらすか
    s_first_idx: int
    for i in range(len(alphabet)):
        if input_s[0] == alphabet[i]:
            s_first_idx = i
            break
    # logger.info(f"{s_first_idx=}")

    t_first_offset: int
    for i in range(s_first_idx, s_first_idx + len(alphabet)):
        if input_t[0] == alphabet[i % len(alphabet)]:
            t_first_offset = i - s_first_idx
            break
    # logger.info(f"{t_first_offset=}")

    s_slide_char_list: List[str] = [
        alphabet[(alphabet_dict[input_s[i]] + t_first_offset) % len(alphabet)] for i in range(len(input_s))
    ]
    s_slided: str = "".join(s_slide_char_list)

    if input_t == s_slided:
        print("Yes")
        return

    print("No")


if __name__ == "__main__":
    main()
