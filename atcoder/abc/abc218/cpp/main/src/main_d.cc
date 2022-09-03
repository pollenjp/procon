#include <iostream>
#include <queue>
#include <vector>

#include "d/solver.h"

int main() {
  int n(0);  // 4 <= n <= 2,000

  std::cin >> n;

  std::vector<std::pair<int, int>> coord_pair_vec(n);

  for (auto &coord_pair : coord_pair_vec) {
    std::cin >> coord_pair.first >> coord_pair.second;
  }

  std::cout << abc218::solve(coord_pair_vec) << std::endl;

  return EXIT_SUCCESS;
}
