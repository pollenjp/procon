#include <algorithm>
#include <exception>
#include <iostream>
#include <utility>
#include <vector>

namespace abc217 {
void solve(const int l, const std::vector<std::pair<int, int>> &q_vec) {
  std::vector<int> cut_x_idx_vec;
  cut_x_idx_vec.push_back(0);
  cut_x_idx_vec.push_back(l);

  for (auto &q_vec_i : q_vec) {
    auto c_i = q_vec_i.first;
    auto x_i = q_vec_i.second;
    if (c_i == 1) {
      cut_x_idx_vec.insert(std::upper_bound(cut_x_idx_vec.begin(), cut_x_idx_vec.end(), x_i), x_i);
    } else if (c_i == 2) {
      auto itr = std::upper_bound(cut_x_idx_vec.begin(), cut_x_idx_vec.end(), x_i);
      auto val = (*itr) - *(itr - 1);
      std::cout << val << std::endl;
    } else {
      throw std::runtime_error("unexpected value: c_i");
    }
  }
}
}  // namespace abc217
