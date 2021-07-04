#include <iostream>
#include <vector>

#include "c/solver.h"

int main() {
  int N;        // 1 <= N <= 2*10^5
  long long K;  // 1 <= K <= 10^18
  std::cin >> N >> K;
  std::vector<int> vec(N, 0);
  for (auto itr = vec.begin(); itr != vec.end(); itr++) {
    std::cin >> *itr;
  }
  auto answer = solve(N, K, vec);
  for (auto ans : answer) {
    std::cout << ans << std::endl;
  }
  return EXIT_SUCCESS;
}
