#pragma once

#include <vector>

namespace {

int solve(const std::vector<int> &a_vector) {
  std::vector<int> b_vector(a_vector);
  int times(0);
  bool end_flag(false);
  while (true) {
    for (auto iter = b_vector.begin(); iter != b_vector.end(); iter++) {
      if (*iter % 2 != 0) {
        end_flag = true;
        break;
      }
      *iter = *iter / 2;
    }

    if (end_flag) {
      break;
    }

    times++;
  }
  return times;
}

}  // namespace
