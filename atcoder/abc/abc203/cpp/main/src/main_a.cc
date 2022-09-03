#include <iostream>

#include "a/solver.h"

int main() {
  int a, b, c;
  std::cin >> a >> b >> c;

  std::cout << solve(a, b, c) << std::endl;
  return EXIT_SUCCESS;
}
