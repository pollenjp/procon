#include <algorithm>
#include <vector>

namespace {

int solve(const int s, const int t) {
  std::vector<int> a_vec(s + 1, 0), b_vec(s + 1, 0), c_vec(s + 1, 0);
  for (int i = 0; i <= s; ++i) {
    a_vec[i] = i;
    b_vec[i] = i;
    c_vec[i] = i;
  }

  int cnt = 0;

  for (auto itr_a = a_vec.begin(), end = a_vec.end(); itr_a != end; ++itr_a) {
    for (auto itr_b = b_vec.begin(), end = b_vec.end(); itr_b != end; ++itr_b) {
      for (auto itr_c = c_vec.begin(), end = c_vec.end(); itr_c != end;
           ++itr_c) {
        if ((*itr_a + *itr_b + *itr_c <= s) &&
            ((*itr_a) * (*itr_b) * (*itr_c) <= t)) {
          cnt++;
        }
      }
    }
  }

  return cnt;
}
}  // namespace
