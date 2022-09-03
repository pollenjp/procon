
#include <cmath>
#include <iostream>

namespace {

int solve(const int a, const int b, const int c, const int d) {
  int tmp = c * d - b;
  if (tmp > 0) {
    return static_cast<int>(std::ceil(static_cast<double>(a) / tmp));
  }
  return -1;
}
}  // namespace
