#include <iostream>
#include <vector>

#include "b/solver.h"

int main() {
  int s, t;  // 2 <= N <= 2*10^5;

  std::cin >> s >> t;

  std::cout << solve(s, t) << std::endl;

  return EXIT_SUCCESS;
}
