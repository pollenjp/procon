import logging
from logging import getLogger
from typing import List, Tuple

logging.basicConfig(
    format="[%(asctime)s][%(levelname)s][%(filename)s:%(lineno)d] - %(message)s",
    level=logging.WARNING,
)
logger = getLogger(__name__)
logger.setLevel(logging.INFO)


def is_valid_tringle(point_a, point_b, point_c) -> bool:

    ab_vec_x = point_b[0] - point_a[0]
    ab_vec_y = point_b[1] - point_a[1]
    ac_vec_x = point_c[0] - point_a[0]
    ac_vec_y = point_c[1] - point_a[1]

    cross_product_z = ab_vec_x * ac_vec_y - ab_vec_y * ac_vec_x
    if cross_product_z == 0:
        return False
    return True


def main():

    input_n: int = int(input().rstrip())
    xy_list: List[Tuple[int, int]] = []
    for _ in range(input_n):
        xy_list.append(tuple(map(int, input().rstrip().split())))

    valid_triangles_cnt: int = 0
    for point_a_idx in range(input_n - 2):
        point_a = xy_list[point_a_idx]
        for point_b_idx in range(point_a_idx + 1, input_n - 1):
            point_b = xy_list[point_b_idx]
            for point_c_idx in range(point_b_idx + 1, input_n):
                point_c = xy_list[point_c_idx]
                if is_valid_tringle(point_a, point_b, point_c):
                    valid_triangles_cnt += 1

    print(valid_triangles_cnt)


if __name__ == "__main__":
    main()
