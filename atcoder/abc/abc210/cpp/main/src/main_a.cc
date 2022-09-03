#include <iostream>

#include "a/solver.h"

int main() {
  int n, a, x, y;
  std::cin >> n >> a >> x >> y;

  std::cout << solve(n, a, x, y) << std::endl;
  return EXIT_SUCCESS;
}
