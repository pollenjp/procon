#include <algorithm>
#include <vector>

namespace {

std::vector<int> solve(const std::vector<int> &s_vec,
                       const std::vector<int> &t_vec) {
  size_t vec_size = t_vec.size();
  auto min_time_vec = t_vec;
  auto min_iter = std::min_element(t_vec.begin(), t_vec.end());
  size_t min_index = std::distance(t_vec.begin(), min_iter);

  for (size_t i = 0; i < vec_size; i++) {
    size_t idx = (min_index + i) % vec_size;
    size_t idx_minus1;
    if (idx == 0) {
      idx_minus1 = vec_size - 1;
    } else {
      idx_minus1 = idx - 1;
    }
    min_time_vec[idx] = std::min(min_time_vec[idx],
                                 s_vec[idx_minus1] + min_time_vec[idx_minus1]);
  }

  return min_time_vec;
}
}  // namespace
