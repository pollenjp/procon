#include <iostream>

#include "b/solver.h"

int main() {
  int a, b, c, d;
  std::cin >> a >> b >> c >> d;

  std::cout << solve(a, b, c, d) << std::endl;
  return EXIT_SUCCESS;
}
