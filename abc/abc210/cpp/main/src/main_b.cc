#include <iostream>
#include <string>

#include "b/solver.h"

int main() {
  int N;  // 1 <= N <= 10^5
  std::string S;
  std::cin >> N >> S;

  std::cout << solve(N, S) << std::endl;
  return EXIT_SUCCESS;
}
