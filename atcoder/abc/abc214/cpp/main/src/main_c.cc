#include <iostream>
#include <vector>

#include "c/solver.h"

int main() {
  int n;  // 1 <= n <= 2 * 10^5
  std::cin >> n;

  std::vector<int> s_vec(n), t_vec(n);

  for (auto itr = s_vec.begin(); itr != s_vec.end(); ++itr) {
    std::cin >> *itr;
  }
  for (auto itr = t_vec.begin(); itr != t_vec.end(); ++itr) {
    std::cin >> *itr;
  }

  auto min_time_vec = solve(s_vec, t_vec);
  for (auto itr = min_time_vec.begin(); itr != min_time_vec.end(); ++itr) {
    std::cout << *itr << std::endl;
  }

  return EXIT_SUCCESS;
}
