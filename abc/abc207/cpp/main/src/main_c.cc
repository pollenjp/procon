#include <iostream>
#include <vector>

#include "c/solver.h"

int main() {
  int N;
  std::cin >> N;
  std::vector<std::vector<int>> vec(N, {0, 0, 0});
  for (auto itr = vec.begin(); itr != vec.end(); itr++) {
    std::cin >> (*itr)[0] >> (*itr)[1] >> (*itr)[2];
  }
  std::cout << solve(vec) << std::endl;
  return EXIT_SUCCESS;
}
