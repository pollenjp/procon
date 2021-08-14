#include <algorithm>
#include <vector>

namespace {

void solve(std::vector<std::pair<int, int>> &x_vec) {
  std::vector<std::pair<int, int>> a_vec(x_vec.size(), {0, 0});
  std::vector<std::pair<int, int>> b_vec(x_vec.size(), {0, 0});
  for (std::size_t i = 0; i < a_vec.size(); ++i) {
    a_vec[i].first = i;
    a_vec[i].second = x_vec[i].first;
    b_vec[i].first = i;
    b_vec[i].second = x_vec[i].second;
  }

  std::sort(a_vec.begin(), a_vec.end(),
            [](const std::pair<int, int> &a, const std::pair<int, int> &b) {
              return (a.second < b.second);
            });
  std::sort(b_vec.begin(), b_vec.end(),
            [](const std::pair<int, int> &a, const std::pair<int, int> &b) {
              return (a.second < b.second);
            });

  std::size_t j;
  j = 0;
  for (std::size_t i = 0; i < x_vec.size(); ++i) {
    if (i != 0) {
      if (a_vec[i].second == a_vec[i - 1].second) {
        j--;
      }
    }
    x_vec[a_vec[i].first].first = j + 1;
    j++;
  }

  j = 0;
  for (std::size_t i = 0; i < x_vec.size(); ++i) {
    if (i != 0) {
      if (b_vec[i].second == b_vec[i - 1].second) {
        j--;
      }
    }
    x_vec[b_vec[i].first].second = j + 1;
    j++;
  }
}
}  // namespace
