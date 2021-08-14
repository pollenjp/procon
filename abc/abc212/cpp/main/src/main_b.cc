#include <iostream>
#include <string>

#include "b/solver.h"

int main() {
  std::string s;

  std::cin >> s;

  if (solve(s)) {
    std::cout << "Weak" << std::endl;
  } else {
    std::cout << "Strong" << std::endl;
  }

  return EXIT_SUCCESS;
}
