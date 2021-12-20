# import logging
# from logging import getLogger
from typing import List

# logging.basicConfig(
#     format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
#     level=logging.WARNING,
# )
# logger = getLogger(__name__)
# logger.setLevel(logging.INFO)


def main():

    input_h, input_w = map(int, input().rstrip().split())

    tile_list: List[List[str]] = []
    for h in range(input_h):
        tile_list.append(input().rstrip() + "#")  # 右端を用意
    tile_list.append("#" * (input_w + 1))  # 下端を用意
    WALL = "#"

    def dfs(y, x):
        r_x, r_y = x + 1, y  # right
        b_x, b_y = x, y + 1  # bottom
        route_num = 0
        koho_route = [None, None]
        if tile_list[r_y][r_x] != WALL:
            koho_route[koho_route] = dfs(r_y, r_x)
            route_num += 1
        if tile_list[b_y][b_x] != WALL:
            koho_route[koho_route] = dfs(b_y, b_x)
            route_num += 1
        if route_num == 0:
            return [(y, x)]
        if route_num == 1:
            return [(y, x)] + koho_route[0]
        if len(koho_route[0]) < len(koho_route[1]):
            return [(y, x)] + koho_route[1]  # 最長を返す
        return [(y, x)] + koho_route[0]  # 最長を返す

    route = dfs(0, 0)

    print(f"{len(route)}")


if __name__ == "__main__":
    main()
