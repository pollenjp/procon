#include <iostream>
#include <vector>

#include "c/solver.h"

int main() {
  std::string s;  // 8 <= |s| <= 10^5
  std::cin >> s;
  std::cout << solve(s) << std::endl;
  return EXIT_SUCCESS;
}
