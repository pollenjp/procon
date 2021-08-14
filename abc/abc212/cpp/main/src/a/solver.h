
#include <string>

namespace {

std::string solve(const int a, const int b) {
  if (a > 0 && b == 0) {
    return std::string("Gold");
  } else if (a == 0 && b > 0) {
    return std::string("Silver");
  } else if (a > 0 && b > 0) {
    return std::string("Alloy");
  } else {
    exit(1);
  }
}
}  // namespace
