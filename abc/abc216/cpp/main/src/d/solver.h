#include <queue>
#include <vector>

namespace {
bool solve(const int n, const int m, std::vector<int> k_vec, std::vector<std::queue<int>> a_vec) {
  // index: boll color, value: a vector of top ball indices
  std::vector<std::vector<std::size_t>> top_ball_color_idx_2_pole_indices_vec(n);
  for (std::size_t i = 0; i < a_vec.size(); i++) {  // init
    auto top_ball_color_idx = a_vec[i].front() - 1;
    top_ball_color_idx_2_pole_indices_vec[top_ball_color_idx].push_back(i);
  }

  std::queue<std::size_t> matched_ball_color_idx_queue;
  for (size_t top_ball_color_idx = 0; top_ball_color_idx < top_ball_color_idx_2_pole_indices_vec.size();
       top_ball_color_idx++) {
    if (top_ball_color_idx_2_pole_indices_vec[top_ball_color_idx].size() == 2) {
      matched_ball_color_idx_queue.push(top_ball_color_idx);
    }
  }

  while (!matched_ball_color_idx_queue.empty()) {
    auto matched_ball_color_idx = matched_ball_color_idx_queue.front();
    // std::cout << matched_ball_color_idx << std::endl;
    matched_ball_color_idx_queue.pop();
    for (auto &i : top_ball_color_idx_2_pole_indices_vec[matched_ball_color_idx]) {
      a_vec[i].pop();
      if (!a_vec[i].empty()) {
        auto top_ball_color_idx = a_vec[i].front() - 1;
        // std::cout << top_ball_color_idx << std::endl;
        top_ball_color_idx_2_pole_indices_vec[top_ball_color_idx].push_back(i);
        if (top_ball_color_idx_2_pole_indices_vec[top_ball_color_idx].size() == 2) {
          matched_ball_color_idx_queue.push(top_ball_color_idx);
        }
      }
    }
  }

  for (auto &a_i : a_vec) {
    if (!a_i.empty()) {
      return false;
    }
  }

  return true;
}
}  // namespace
