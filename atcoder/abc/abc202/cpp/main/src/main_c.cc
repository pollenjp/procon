#include <iostream>
#include <vector>

#include "c/solver.h"

int main() {
  int N;
  std::cin >> N;
  std::vector<std::vector<int>> V(3, std::vector<int>(N, 0));
  for (int i = 0; i < 3; i++) {
    for (int j = 0; j < N; j++) {
      std::cin >> V[i][j];
    }
  }

  std::cout << solve(V[0], V[1], V[2]) << std::endl;
  return EXIT_SUCCESS;
}
