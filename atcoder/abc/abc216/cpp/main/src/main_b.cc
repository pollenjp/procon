#include <iostream>
#include <string>
#include <vector>

#include "b/solver.h"

int main() {
  int n;  // 2 <= N <= 10*3
  std::cin >> n;

  std::vector<std::pair<std::string, std::string>> s_pair_vec(n, std::make_pair("", ""));

  for (auto &s_pair : s_pair_vec) {
    std::cin >> s_pair.first >> s_pair.second;
  }

  if (solve(s_pair_vec)) {
    std::cout << "Yes" << std::endl;
  } else {
    std::cout << "No" << std::endl;
  }

  return EXIT_SUCCESS;
}
