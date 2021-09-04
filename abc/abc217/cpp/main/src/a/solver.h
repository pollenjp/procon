#include <algorithm>
#include <exception>
#include <stdexcept>
#include <string>
#include <vector>

#define LOG_BASE 2

namespace abc217 {

std::string solve(const std::string &a, const std::string &b) {
  std::size_t max_len = std::max(a.size(), b.size());
  for (std::size_t i = 0; i < max_len; i++) {
    if (a.size() <= i) {
      return "Yes";
    } else if (b.size() <= i) {
      return "No";
    }

    if (a[i] < b[i]) {
      return "Yes";
    } else if (a[i] > b[i]) {
      return "No";
    }
  }

  throw std::runtime_error("Not matched!");
}
}  // namespace abc217
