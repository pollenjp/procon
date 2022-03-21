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

    input_n: int = int(input().rstrip())
    input_t: str = input().rstrip()

    current_position: List[int] = [0, 0]
    current_direction: Tuple[int, int] = (1, 0)

    for char in input_t:
        if char == "S":
            current_position[0] += current_direction[0]
            current_position[1] += current_direction[1]
        elif char == "R":
            x, y = current_direction
            current_direction = (y, -x)

    x, y = current_position
    print(f"{x} {y}")


if __name__ == "__main__":
    main()
