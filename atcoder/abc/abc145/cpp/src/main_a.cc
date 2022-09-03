#include <iostream>

int solve(int in) { return in * in; }

int main() {
  int r;
  std::cin >> r;
  std::cout << solve(r) << std::endl;
  return EXIT_SUCCESS;
}
