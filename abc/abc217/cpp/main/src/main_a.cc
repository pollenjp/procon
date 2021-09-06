#include <iostream>
#include <stdexcept>
#include <string>

#include "a/solver.h"

int main() {
  try {
    std::string s;
    std::string t;
    std::cin >> s >> t;

    std::cout << abc217::solve(s, t) << std::endl;
    return EXIT_SUCCESS;
  } catch (std::exception& e) {
    std::cerr << "error: " << e.what() << std::endl;
    return EXIT_FAILURE;
  } catch (...) {
    return EXIT_FAILURE;
  }
}
