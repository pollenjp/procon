
#include <algorithm>
#include <iostream>
#include <vector>

namespace {

bool solve(const int a, const int b) {
  int upper_limit(6 * a), lower_limmit(a);
  if (b >= lower_limmit && b <= upper_limit) {
    return true;
  }
  return false;
}
}  // namespace
