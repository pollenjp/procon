#include "solver.h"

namespace abc217 {
std::string solve(const std::string &a, const std::string &b) {
  for (std::size_t i = 0; i < std::max(a.size(), b.size()); i++) {
    if (a.size() <= i) {
      return "Yes";
    }
    if (b.size() <= i) {
      return "No";
    }

    if (a[i] < b[i]) {
      return "Yes";
    }
    if (a[i] > b[i]) {
      return "No";
    }
  }

  throw std::runtime_error("Not matched!");
}
}  // namespace abc217
