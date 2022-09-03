import logging
from dataclasses import dataclass
from logging import getLogger
from typing import List

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


@dataclass
class AssociativeArrayElement:
    pre: int = None
    post: int = None


def main():

    input_n, input_q = map(int, input().rstrip().split(" "))

    query_list: List[List[int]] = []
    for _ in range(input_q):
        query_list.append(list(map(int, input().rstrip().split(" "))))

    trains: List[AssociativeArrayElement] = [AssociativeArrayElement() for _ in range(input_n)]

    q: List[int]
    for q in query_list:
        if q[0] == 1:
            x, y = q[1:]
            x, y = x - 1, y - 1
            trains[x].post = y
            trains[y].pre = x
        elif q[0] == 2:
            x, y = q[1:]
            x, y = x - 1, y - 1
            trains[x].post = None
            trains[y].pre = None
        else:  # q[0] == 3
            x = q[1] - 1
            pre_trains: List[int] = []
            post_trains: List[int] = []

            # pre
            i = x
            while i is not None:
                pre_trains += [i]
                i = trains[i].pre
            # post
            i = x
            while i is not None:
                post_trains += [i]
                i = trains[i].post

            pre_trains.reverse()
            pre_trains = pre_trains[:-1]
            logger.info(f"{pre_trains=}")
            logger.info(f"{post_trains=}")
            oneline_traines: List[int] = pre_trains + post_trains
            target_str: str = f"{len(oneline_traines)} {' '.join([str(i + 1) for i in pre_trains + post_trains])}"
            print(target_str)


if __name__ == "__main__":
    main()
