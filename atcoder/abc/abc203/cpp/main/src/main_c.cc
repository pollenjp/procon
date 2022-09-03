#include <iostream>
#include <vector>

#include "c/solver.h"

int main() {
  int N;
  long long K;
  std::cin >> N >> K;
  std::vector<std::pair<long long, long long>> ab_vectors(N, {0, 0});
  for (auto itr = ab_vectors.begin(); itr != ab_vectors.end(); itr++) {
    // std::cin >> (*itr).first >> (*itr).second;
    long long x, y;
    std::cin >> x >> y;
    (*itr).first = x;
    (*itr).second = y;
  }

  std::cout << solve(N, K, ab_vectors) << std::endl;
  return EXIT_SUCCESS;
}
