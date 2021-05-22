#include <iostream>
#include <string>

#include "b/solver.h"

int main() {
  std::string S;
  std::cin >> S;

  std::cout << solve(S) << std::endl;
  return EXIT_SUCCESS;
}
