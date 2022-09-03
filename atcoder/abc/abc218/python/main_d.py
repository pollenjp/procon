from logging import getLogger
from typing import Dict, List, Tuple

logger = getLogger(__name__)


def main():
    input_n: int = int(input().rstrip())
    coords_list: List[Tuple[int, int]] = []
    for i in range(input_n):
        coords_list.append(tuple(map(int, input().rstrip().split(" "))))

    x_to_coord_idxes_dict: Dict[int, List[int]] = {x_i: [] for x_i, _ in coords_list}
    y_to_coord_idxes_dict: Dict[int, List[int]] = {y_i: [] for _, y_i in coords_list}
    for coord_idx, (x_i, y_i) in enumerate(coords_list):
        x_to_coord_idxes_dict[x_i].append(coord_idx)
        y_to_coord_idxes_dict[y_i].append(coord_idx)

    # upper left -> upper right -> lower right -> lower left の順番で探索する
    # next とは xy を upper left としたときに upper right になりうるもののこと
    next1_coords_indices: List[int] = [y_to_coord_idxes_dict[y_i].copy() for x_i, y_i in coords_list]
    for coord_idx_1, next_coords_idxes in enumerate(next1_coords_indices):
        x_i, y_i = coords_list[coord_idx_1]
        tmp = []
        for coord_idx_2 in next_coords_idxes:
            if coords_list[coord_idx_2][0] > x_i:
                tmp.append(coord_idx_2)
        next1_coords_indices[coord_idx_1] = tmp

    next2_coords_indices: List[int] = [x_to_coord_idxes_dict[x_i].copy() for x_i, y_i in coords_list]
    for coord_idx_1, next_coords_idxes in enumerate(next2_coords_indices):
        x_i, y_i = coords_list[coord_idx_1]
        tmp = []
        for coord_idx_2 in next_coords_idxes:
            if coords_list[coord_idx_2][1] < y_i:
                tmp.append(coord_idx_2)
        next2_coords_indices[coord_idx_1] = tmp

    next3_coords_indices: List[int] = [y_to_coord_idxes_dict[y_i].copy() for x_i, y_i in coords_list]
    for coord_idx_1, next_coords_idxes in enumerate(next3_coords_indices):
        x_i, y_i = coords_list[coord_idx_1]
        tmp = []
        for coord_idx_2 in next_coords_idxes:
            if coords_list[coord_idx_2][0] < x_i:
                tmp.append(coord_idx_2)
        next3_coords_indices[coord_idx_1] = tmp

    next4_coords_indices: List[int] = [x_to_coord_idxes_dict[x_i].copy() for x_i, y_i in coords_list]
    for coord_idx_1, next_coords_idxes in enumerate(next4_coords_indices):
        x_i, y_i = coords_list[coord_idx_1]
        tmp = []
        for coord_idx_2 in next_coords_idxes:
            if coords_list[coord_idx_2][1] > y_i:
                tmp.append(coord_idx_2)
        next4_coords_indices[coord_idx_1] = tmp

    logger.warning(next1_coords_indices)
    logger.warning(next2_coords_indices)
    logger.warning(next3_coords_indices)
    logger.warning(next4_coords_indices)

    cnt: int = 0
    for upper_left_coord_idx, next1_coords_indices in enumerate(next1_coords_indices):
        if len(next1_coords_indices) == 0:
            continue
        for next1_coord_idx in next1_coords_indices:
            for next2_coord_idx in next2_coords_indices[next1_coord_idx]:
                for next3_coord_idx in next3_coords_indices[next2_coord_idx]:
                    for next4_coord_idx in next4_coords_indices[next3_coord_idx]:
                        if next4_coord_idx == upper_left_coord_idx:
                            cnt += 1

    print(cnt)


if __name__ == "__main__":
    main()
