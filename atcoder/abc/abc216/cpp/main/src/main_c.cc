#include <iostream>
#include <string>
#include <vector>

#include "c/solver.h"

int main() {
  long long n;  // 10*18
  std::cin >> n;

  std::cout << solve(n) << std::endl;

  return EXIT_SUCCESS;
}
