import logging
from logging import getLogger

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)

const_mod: int = 998244353


def main():

    x1, y1, x2, y2 = map(int, input().rstrip().split())

    xy1 = (x1, y1)
    xy2 = (x2, y2)
    candidate_set_dict = {xy1: set(), xy2: set()}
    for (x, y) in [xy1, xy2]:
        for offset_x, offset_y in [
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
        ]:
            candidate_set_dict[(x, y)].add((x + offset_x, y + offset_y))

    for (x, y) in candidate_set_dict[xy1]:
        if (x, y) in candidate_set_dict[xy2]:
            print("Yes")
            return
    print("No")


if __name__ == "__main__":
    main()
