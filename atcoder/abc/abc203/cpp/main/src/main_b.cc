#include <iostream>

#include "b/solver.h"

int main() {
  int N, K;
  std::cin >> N >> K;

  std::cout << solve(N, K) << std::endl;
  return EXIT_SUCCESS;
}
