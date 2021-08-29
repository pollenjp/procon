#include <iostream>
#include <vector>

#include "d/solver.h"

int main() {
  int n, m;

  std::cin >> n >> m;

  std::vector<int> k_vec(m);
  std::vector<std::vector<int>> a_vec(m, std::vector<int>());

  for (auto i = 0; i < m; i++) {
    std::cin >> k_vec[i];
    for (auto j = 0; j < k_vec[i]; j++) {
      int a;
      std::cin >> a;
      a_vec[i].push_back(a);
    }
  }

  if (solve(n, m, k_vec, a_vec)) {
    std::cout << "Yes" << std::endl;
  } else {
    std::cout << "No" << std::endl;
  }

  return EXIT_SUCCESS;
}
