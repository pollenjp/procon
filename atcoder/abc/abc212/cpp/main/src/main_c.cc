#include <iostream>
#include <vector>

#include "c/solver.h"

int main() {
  std::size_t n, m;  // 1 <=n, m <= 2*10^5
  std::cin >> n >> m;
  std::vector<int> a(n, 0), b(m, 0);  // 1 <= a[i], a[i] <= 10^9
  for (std::size_t i = 0; i < n; ++i) {
    std::cin >> a[i];
  }
  for (std::size_t i = 0; i < m; ++i) {
    std::cin >> b[i];
  }

  std::cout << solve(a, b) << std::endl;
  return EXIT_SUCCESS;
}
