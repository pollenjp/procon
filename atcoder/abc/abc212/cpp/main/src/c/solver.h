#include <algorithm>
#include <cmath>
#include <string>
#include <vector>

namespace {

#define MAX_NUM static_cast<int>(1e9)

int solve(std::vector<int> &a, std::vector<int> &b) {
  std::sort(a.begin(), a.end());
  std::sort(b.begin(), b.end());

  int minimum(MAX_NUM);
  std::size_t i(0), j(0);

  for (i = 0, j = 0; i < a.size(); ++i) {
    int x = a[i];
    int y, tmp1(0), tmp2(MAX_NUM + 1);
    while (true) {
      y = b[j];
      tmp1 = std::abs(x - y);

      if (tmp1 < tmp2) {
        tmp2 = tmp1;
      }
      if (x < y) {
        break;
      }

      if (j == b.size() - 1) {
        break;
      }
      j++;
    }
    if (tmp2 < minimum) {
      minimum = tmp2;
    }
  }

  return minimum;
}
}  // namespace
