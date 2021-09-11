#include <algorithm>
#include <vector>

namespace abc218 {
int solve(std::vector<std::pair<int, int>> &coord_pair_vec) {
  std::sort(coord_pair_vec.begin(), coord_pair_vec.end());

  int cnt(0);

  for (auto &coord_pair_1 : coord_pair_vec) {
    for (auto &coord_pair_2 : coord_pair_vec) {
      if (coord_pair_1.first < coord_pair_2.first && coord_pair_1.second < coord_pair_2.second) {
        auto ret = std::binary_search(coord_pair_vec.begin(), coord_pair_vec.end(),
                                      std::make_pair(coord_pair_1.first, coord_pair_2.second));
        ret = ret && std::binary_search(coord_pair_vec.begin(), coord_pair_vec.end(),
                                        std::make_pair(coord_pair_2.first, coord_pair_1.second));
        if (ret) {
          cnt++;
        }
      }
    }
  }

  return cnt;
}
}  // namespace abc218
