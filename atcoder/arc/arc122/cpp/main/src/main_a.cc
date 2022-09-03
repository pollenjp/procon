#include <exception>
#include <iostream>
#include <vector>

#include "a/solver.h"

int main() {
  try {
    int64_t n(0);
    std::cin >> n;

    std::vector<int64_t> a_vector(n, 0);

#pragma unroll 5
    for (auto &a_i : a_vector) {
      std::cin >> a_i;
    }

    std::cout << arc122::solve(n, a_vector) << std::endl;
    return EXIT_SUCCESS;
  } catch (const std::exception &e) {
    std::cerr << "Exception: " << e.what() << std::endl;
  } catch (...) {
    std::cerr << "Unknown exception" << std::endl;
  }
  return EXIT_FAILURE;
}
