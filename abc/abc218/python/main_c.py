from logging import getLogger
from typing import List

import numpy as np

logger = getLogger(__name__)


def trim_grid(grid: np.ndarray) -> np.ndarray:
    row_min, col_min, row_max, col_max = 0, 0, 0, 0
    for row_idx in range(grid.shape[0]):
        if np.count_nonzero(grid[row_idx]) > 0:
            row_min = row_idx
            break
    for row_idx in reversed(range(grid.shape[0])):
        if np.count_nonzero(grid[row_idx]) > 0:
            row_max = row_idx
            break
    for col_idx in range(grid.shape[1]):
        if np.count_nonzero(grid[:, col_idx]) > 0:
            col_min = col_idx
            break
    for col_idx in reversed(range(grid.shape[1])):
        if np.count_nonzero(grid[:, col_idx]) > 0:
            col_max = col_idx
            break

    return grid[row_min : row_max + 1, col_min : col_max + 1]


def main():
    input_n: int = int(input().rstrip())
    grid_s: List[str] = []
    grid_t: List[str] = []
    for i in range(input_n):
        grid_s.append(input().rstrip())
    for i in range(input_n):
        grid_t.append(input().rstrip())

    # convert to 0, 1
    grid_s_num: List[List[int]] = []
    for line in grid_s:
        tmp_list = []
        for val in line:
            tmp_list.append(1 if val == "#" else 0)
        grid_s_num.append(tmp_list)

    grid_t_num: List[List[int]] = []
    for line in grid_t:
        tmp_list = []
        for val in line:
            tmp_list.append(1 if val == "#" else 0)
        grid_t_num.append(tmp_list)

    grid_s_num_trimmed_ndarray: np.ndarray = trim_grid(grid=np.array(grid_s_num))
    grid_t_num_trimmed_ndarray: np.ndarray = trim_grid(grid=np.array(grid_t_num))

    # 4 の回転方向でチェック
    match_flag = False
    for k in range(4):
        s_rotated = np.rot90(grid_s_num_trimmed_ndarray, k=k)
        logger.warning("s")
        logger.warning(s_rotated)
        logger.warning("t")
        logger.warning(grid_t_num_trimmed_ndarray)
        if not (s_rotated.shape == grid_t_num_trimmed_ndarray.shape):
            continue
        if np.all(s_rotated == grid_t_num_trimmed_ndarray):
            match_flag = True
            break

    if match_flag:
        print("Yes")
    else:
        print("No")


if __name__ == "__main__":
    main()
