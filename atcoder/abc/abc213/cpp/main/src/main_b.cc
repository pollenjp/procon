#include <iostream>
#include <vector>

#include "b/solver.h"

int main() {
  int n;  // 2 <= N <= 2*10^5;

  std::cin >> n;
  std::vector<int> a_vec(n, 0);

  for (auto itr = a_vec.begin(); itr != a_vec.end(); ++itr) {
    std::cin >> *itr;
  }

  std::cout << solve(a_vec) << std::endl;

  return EXIT_SUCCESS;
}
