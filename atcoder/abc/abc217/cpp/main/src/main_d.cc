#include <iostream>
#include <queue>
#include <vector>

#include "d/solver.h"

int main() {
  int l;  // 1 <= l <= 10^9
  int q;  // 1 <= q <= 2*10^5

  std::cin >> l >> q;

  std::vector<std::pair<int, int>> q_vec(q);

  for (auto &q_vec_i : q_vec) {
    std::cin >> q_vec_i.first >> q_vec_i.second;
  }

  abc217::solve(l, q_vec);

  return EXIT_SUCCESS;
}
