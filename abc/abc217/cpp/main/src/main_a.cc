#include <iostream>
#include <string>

#include "a/solver.h"

int main() {
  std::string s, t;
  std::cin >> s >> t;

  std::cout << abc217::solve(s, t) << std::endl;
  return EXIT_SUCCESS;
}
