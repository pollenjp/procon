#include <iostream>
#include <vector>

#include "b/solver.h"

int main() {
  int N;
  std::cin >> N;

  std::vector<int> a_vector(N);
  for (auto i = 0; i < N; i++) {
    std::cin >> a_vector.at(i);
  }

  std::cout << solve(a_vector) << std::endl;
  return EXIT_SUCCESS;
}
