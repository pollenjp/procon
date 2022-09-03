#include <iostream>
#include <vector>
#include <string>

#include "c/solver.h"

int main() {
  std::string s;
  int k;
  std::cin >> s >> k;

  std::cout << solve(s, k) << std::endl;

  return EXIT_SUCCESS;
}
