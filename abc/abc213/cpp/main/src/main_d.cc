#include <iostream>
#include <vector>

#include "d/solver.h"

int main() {
  int n;
  std::cin >> n;

  std::vector<std::pair<int, int>> x_vec(n);

  for (auto itr = x_vec.begin(); itr != x_vec.end(); ++itr) {
    std::cin >> itr->first >> itr->second;
  }

  auto answer_vec = solve(n, x_vec);
  std::cout << answer_vec.size() << std::endl;

  for (auto itr = answer_vec.begin(); itr != answer_vec.end(); ++itr) {
    std::cout << (*itr) + 1;
  }
  std::cout << std::endl;
  return EXIT_SUCCESS;
}
