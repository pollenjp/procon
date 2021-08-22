#include <iostream>
#include <vector>

#include "a/solver.h"

int main() {
  int n, m;
  std::cin >> n >> m;

  std::vector<int> s_vec(n, 0);
  std::vector<int> t_vec(m, 0);

  for (auto &s_i : s_vec) {
    std::cin >> s_i;
  }

  for (auto &t_i : t_vec) {
    std::cin >> t_i;
  }

  std::cout << solve(s_vec, t_vec) << std::endl;
  return EXIT_SUCCESS;
}
