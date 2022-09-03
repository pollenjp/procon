
#include <algorithm>
#include <iostream>
#include <vector>

int BOARDER = 206;

namespace {

int solve(const int a, const int b, const int c) {
  std::vector<int> vec({a, b, c});
  std::sort(vec.begin(), vec.end());
  return *(vec.end() - 2) + *(vec.end() - 1);
}
}  // namespace
