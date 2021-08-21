#include <iostream>
#include <string>
#include <vector>

#include "d/solver.h"

int main() {
  int n, m;

  std::cin >> n >> m;

  std::vector<int> a(n);

  for (auto itr = a.begin(); itr != a.end(); ++itr) {
    std::cin >> *itr;
  }

  auto valid_nums = solve(n, m, a);
  std::cout << valid_nums.size() << std::endl;
  for (auto itr = valid_nums.begin(); itr != valid_nums.end(); ++itr) {
    std::cout << *itr << std::endl;
  }

  return EXIT_SUCCESS;
}
