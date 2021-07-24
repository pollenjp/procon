#include <iostream>
#include <string>
#include <vector>

#include "b/solver.h"

int main() {
  int N = 4;
  std::vector<std::string> s_vec(N, "");
  for (auto itr = s_vec.begin(); itr != s_vec.end(); ++itr) {
    std::cin >> (*itr);
  }

  if (solve(s_vec)) {
    std::cout << "Yes" << std::endl;
  } else {
    std::cout << "No" << std::endl;
  }
  return EXIT_SUCCESS;
}
