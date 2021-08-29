#include <algorithm>
#include <iostream>
#include <map>
#include <vector>

namespace {
bool solve(const int n, const int m, std::vector<int> k_vec, std::vector<std::vector<int>> a_vec) {
  // remove empty vectors
  // for (std::size_t i = a_vec.size() - 1; i != -1; i--) {
  // if (a_vec[i].empty()) {
  // a_vec.erase(a_vec.begin() + i);
  // }
  // }

  // index: boll color, value: a vector of top ball indices
  std::vector<std::vector<std::size_t>> top_ball_color_to_pole_indices_vec(n);
  std::vector<std::size_t> indices_vec;
  for (std::size_t i = 0; i < a_vec.size(); i++) {
    if (a_vec[i].empty()) {
      continue;
    }
    indices_vec.push_back(i);
  }
  while (!indices_vec.empty()) {
    bool b_found = false;
    size_t size_of_beginning = indices_vec.size();
    for (size_t i_1 = 0; i_1 < size_of_beginning; i_1++) {
      auto idx_1 = indices_vec[i_1];
      int color = a_vec[idx_1][0];
      top_ball_color_to_pole_indices_vec[color - 1].push_back(idx_1);
      // for (auto &tmp : top_ball_color_to_pole_indices_vec[color - 1]) {
      //   std::cout << "loop: " << idx_1 << ", index: " << tmp << ", color_indices: " << color - 1 << std::endl;
      // }
      if (top_ball_color_to_pole_indices_vec[color - 1].size() == 2) {
        for (auto &idx_2 : top_ball_color_to_pole_indices_vec[color - 1]) {
          // std::cout << idx_2 << std::endl;
          a_vec[idx_2].erase(a_vec[idx_2].begin());
          if (!a_vec[idx_2].empty()) {
            if (!(std::find(indices_vec.begin() + i_1 + 1, indices_vec.begin() + size_of_beginning, idx_2) !=
                  indices_vec.begin() + size_of_beginning)) {
              indices_vec.push_back(idx_2);
            }
          }
        }

        // もう使わないので削除
        top_ball_color_to_pole_indices_vec[color - 1].clear();
        top_ball_color_to_pole_indices_vec[color - 1].shrink_to_fit();

        b_found = true;
        if (i_1 == 0) {
          indices_vec.erase(indices_vec.begin());
        } else {
          indices_vec.erase(indices_vec.begin(), indices_vec.begin() + i_1 + 1);
        }
        break;
      }
    }
    if (!b_found) {
      return false;
    }
  }

  return true;
}
}  // namespace
