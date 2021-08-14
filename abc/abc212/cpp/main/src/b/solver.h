#include <iostream>
#include <string>
#include <vector>
#define VEC_NUM 4

namespace {

bool solve(const std::string &s) {
  std::vector<unsigned int> x_vec(VEC_NUM, 0);

  if (s.size() != VEC_NUM) {
    exit(1);
  }

  for (std::size_t i = 0; i < s.size(); ++i) {
    x_vec[i] = static_cast<unsigned int>(s[i] - '0');
  }

  // すべて同じ値

  unsigned int counter(0);
  for (std::size_t i = 0; i < x_vec.size() - 1; i++) {
    if (x_vec[i] != x_vec[i + 1]) {
      break;
    }
    counter++;
  }

  if (counter == x_vec.size() - 1) {
    return true;
  }

  // 条件2

  counter = 0;
  for (std::size_t i = 0; i < x_vec.size() - 1; i++) {
    if ((x_vec[i] + 1) % 10 == x_vec[(i + 1) % x_vec.size()]) {
      counter++;
    }
  }
  if (counter == x_vec.size() - 1) {
    return true;
  }

  return false;
}
}  // namespace
