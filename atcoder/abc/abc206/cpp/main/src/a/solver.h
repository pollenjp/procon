
#include <cmath>
#include <iostream>
#include <stdexcept>
#include <string>

int BOARDER = 206;

namespace {
int BOARDER = 206;
std::string solve(const int N) {
  int val;
  val = static_cast<int>(std::floor(static_cast<float>(N) * 1.08f));

  if (val < BOARDER) {
    return "Yay!";
  } else if (val == BOARDER) {
    return "so-so";
  } else {
    return ":(";
  }
}
}  // namespace
