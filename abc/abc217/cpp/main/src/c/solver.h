#include <vector>

namespace abc217 {

template <class T>
std::vector<T> solve(const std::vector<T> p_vec) {
  std::vector<T> target_vec(p_vec.size(), 0);

  for (std::size_t i = 0; i < p_vec.size(); i++) {
    auto p_i = p_vec[i];
    target_vec[p_i - 1] = i + 1;
  }

  return target_vec;
}
}  // namespace abc217
