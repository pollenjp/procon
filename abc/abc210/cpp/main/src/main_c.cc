#include <iostream>
#include <vector>

#include "c/solver.h"

int main() {
  int K, N;  // 3 * 10^5

  std::cin >> N >> K;

  std::vector<int> candies(N, 0);  // 1 <= c_i <= 10^9

  for (auto itr = candies.begin(); itr != candies.end(); itr++) {
    std::cin >> *itr;
  }
  std::cout << solve(N, K, candies) << std::endl;
  return EXIT_SUCCESS;
}
