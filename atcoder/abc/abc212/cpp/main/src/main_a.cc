#include <iostream>

#include "a/solver.h"

int main() {
  int a, b;
  std::cin >> a >> b;

  std::cout << solve(a, b) << std::endl;
  return EXIT_SUCCESS;
}
