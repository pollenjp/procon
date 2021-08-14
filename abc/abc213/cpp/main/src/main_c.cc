#include <iostream>
#include <vector>

#include "c/solver.h"

int main() {
  int h, w, n;
  std::cin >> h >> w >> n;

  std::vector<std::pair<int, int>> x_vec(n);

  for (auto itr = x_vec.begin(); itr != x_vec.end(); ++itr) {
    std::cin >> itr->first >> itr->second;
  }

  solve(x_vec);
  for (auto itr = x_vec.begin(); itr != x_vec.end(); ++itr) {
    std::cout << itr->first << " " << itr->second << std::endl;
  }
  return EXIT_SUCCESS;
}
