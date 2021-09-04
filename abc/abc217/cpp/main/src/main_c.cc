#include <iostream>
#include <vector>

#include "c/solver.h"

int main() {
  int n(0);  // 2*10*5
  std::cin >> n;

  std::vector<int> p_vec(n, 0);

  for (auto &p_i : p_vec) {
    std::cin >> p_i;
  }

  for (auto &val : abc217::solve(p_vec)) {
    std::cout << val << " ";
  }
  std::cout << std::endl;

  return EXIT_SUCCESS;
}
