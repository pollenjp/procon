#include <iostream>

#include "a/solver.h"

int main() {
  int a, b;
  std::cin >> a >> b;

  if (solve(a, b)) {
    std::cout << "Yes" << std::endl;

  } else {
    std::cout << "No" << std::endl;
  }
  return EXIT_SUCCESS;
}
