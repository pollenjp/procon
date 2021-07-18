#include <iostream>

#include "a/solver.h"

int main() {
  long long a1, a2, a3;  // 10^15

  std::cin >> a1 >> a2 >> a3;

  std::cout << solve(a1, a2, a3) << std::endl;
  return EXIT_SUCCESS;
}
