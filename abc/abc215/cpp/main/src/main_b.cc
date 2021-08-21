#include <iostream>
#include <vector>

#include "b/solver.h"

int main() {
  long long n;  // 2 <= N <= 2*10^18

  std::cin >> n;

  std::cout << solve(n) << std::endl;

  return EXIT_SUCCESS;
}
