#pragma once

#include <vector>

namespace {

long long solve(const std::vector<int> &a_vector,
                const std::vector<int> &b_vector,
                const std::vector<int> &c_vector) {
  int N = a_vector.size();
  long long counter(0);
  std::vector<int> count_vector(N, 0);
  for (int j = 0; j < N; j++) {
    count_vector[b_vector[c_vector[j] - 1] - 1]++;
  }

  for (int i = 0; i < N; i++) {
    counter += static_cast<long long>(count_vector[a_vector[i] - 1]);
  }
  return counter;
}

}  // namespace
