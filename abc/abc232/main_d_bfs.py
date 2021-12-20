import collections
import logging
from logging import getLogger
from typing import List, Set, Tuple

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def main():

    input_h, input_w = map(int, input().rstrip().split())

    wall_char: str = "#"

    # 右と下に壁追加

    # size: (h + 1, w + 1)
    tile_list: List[str] = []
    for h in range(input_h):
        tile_list.append(input().rstrip() + wall_char)
    tile_list.append(wall_char * (input_w + 1))

    # size: (h + 1, w + 1)
    max_cost_tile: List[List[int]] = [[0] * (input_w + 1) for _ in range(input_h + 1)]
    max_cost_tile[0][0] = 1

    vertices_searched: Set[Tuple[int, int]] = set()

    def bfs(y: int = 0, x: int = 0) -> int:
        max_cost: int = 1
        queue: collections.deque = collections.deque([(y, x)])
        while queue:
            # logger.info(f"{queue=}")
            y, x = queue.popleft()

            for offset_y, offset_x in [(1, 0), (0, 1)]:
                next_y, next_x = y + offset_y, x + offset_x
                if tile_list[next_y][next_x] != wall_char:
                    max_cost_tile[next_y][next_x] = max(max_cost_tile[next_y][next_x], max_cost_tile[y][x] + 1)
                    max_cost = max(max_cost, max_cost_tile[next_y][next_x])
                    if (next_y, next_x) not in vertices_searched:
                        vertices_searched.add((next_y, next_x))
                        queue.append((next_y, next_x))
        return max_cost

    max_cost: int = bfs(0, 0)
    print(f"{max_cost}")


if __name__ == "__main__":
    main()
