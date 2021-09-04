#include <iostream>
#include <string>

#include "b/solver.h"

int main() {
  std::string s1, s2, s3;
  std::cin >> s1 >> s2 >> s3;

  std::cout << abc217::solve(s1, s2, s3) << std::endl;

  return EXIT_SUCCESS;
}
