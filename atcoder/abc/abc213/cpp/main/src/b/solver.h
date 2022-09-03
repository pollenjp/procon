#include <algorithm>
#include <vector>

namespace {

int solve(const std::vector<int> &a_vec) {
  std::vector<std::pair<int, int>> x_vec(a_vec.size(), {0, 0});

  for (std::size_t i = 0; i < a_vec.size(); ++i) {
    x_vec[i].first = i;
    x_vec[i].second = a_vec[i];
  }

  std::sort(x_vec.begin(), x_vec.end(),
            [](const std::pair<int, int> &a, const std::pair<int, int> &b) {
              return (a.second < b.second);
            });
  return x_vec[x_vec.size() - 2].first + 1;
}
}  // namespace
