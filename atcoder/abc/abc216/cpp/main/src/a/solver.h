#include <algorithm>
#include <exception>
#include <vector>
#include <string>

#define LOG_BASE 2

namespace {

void solve(const float a) {
  if (a < 0 ) {
    throw std::runtime_error("x must be non-negative");
  }
  int x = int(a);
  float y = a - x;

  if (0.0 <= y && y < 0.25){
    std::cout << x << "-" << std::endl;
  } else if (0.25 <= y && y <= 0.65){
    std::cout << x  << std::endl;
  } else if (0.65 < y && y < 1.05){
    std::cout << x << "+" << std::endl;
  } else {
    throw std::runtime_error("x must be in [0, 1)");
  }
}
}  // namespace
