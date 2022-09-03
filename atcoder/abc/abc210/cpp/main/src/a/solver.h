
#include <iostream>

namespace {

int solve(const int n, const int a, const int x, const int y) {
  int left = n - a;
  if (left < 0) {
    return n * x;
  } else {
    return a * x + left * y;
  }
}
}  // namespace
