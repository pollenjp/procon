#include <algorithm>
#include <exception>
#include <iostream>
#include <set>
#include <utility>
#include <vector>

namespace abc217 {
void solve(const int l, const std::vector<std::pair<int, int>> &q_vec) {
  std::set<int> cut_x_idx_set;
  cut_x_idx_set.insert(0);
  cut_x_idx_set.insert(l);

  for (const auto &q_vec_i : q_vec) {
    auto c_i = q_vec_i.first;
    auto x_i = q_vec_i.second;
    if (c_i == 1) {
      cut_x_idx_set.insert(x_i);
    } else if (c_i == 2) {
      auto itr = cut_x_idx_set.lower_bound(x_i);
      auto val = (*itr) - *(std::prev(itr, 1));
      std::cout << val << std::endl;
    } else {
      throw std::runtime_error("unexpected value: c_i");
    }
  }
}
}  // namespace abc217
