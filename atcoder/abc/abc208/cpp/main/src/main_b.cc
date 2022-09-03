#include <iostream>

#include "b/solver.h"

int main() {
  int p;  // 1 <= P <= 10^7
  std::cin >> p;

  std::cout << solve(p) << std::endl;
  return EXIT_SUCCESS;
}
