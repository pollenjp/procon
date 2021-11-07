#include <glog/logging.h>

#include <algorithm>
#include <numeric>
#include <string>
#include <utility>
#include <vector>

namespace atcoder {

namespace abc226 {

namespace b {

int32_t solve(std::vector<std::vector<int32_t>> const &l_list_list) {
  std::vector<int32_t> sum_list(l_list_list.size());

  for (std::size_t i = 0; i < l_list_list.size(); ++i) {
    sum_list[i] = std::accumulate(l_list_list[i].begin(), l_list_list[i].end(), 0);
  }

  int32_t cnt_duplicate(0);
  for (std::size_t i = 0; i < l_list_list.size() - 1; ++i) {
    for (std::size_t j = i + 1; j < l_list_list[i].size(); ++j) {
      if (sum_list[i] == sum_list[j] && l_list_list[i].size() == l_list_list[j].size()) {
        // element check
        bool is_same(true);
        for (std::size_t k = 0; k < l_list_list[i].size(); ++k) {
          if (l_list_list[i][k] != l_list_list[j][k]) {
            is_same = false;
            break;
          }
        }
        if (is_same) {
          cnt_duplicate++;
        }
      }
    }
  }
  return static_cast<int32_t>(l_list_list.size()) - cnt_duplicate;
}

}  // namespace b

}  // namespace abc226

}  // namespace atcoder
