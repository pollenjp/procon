#include <iostream>

namespace {

long long solve(const long long a1, const long long a2, const long long a3) {
  long long answer(0);
  long long quotient;
  long long b1(a2 - a1), b2(a3 - a2);
  long long diff = b2 - b1;
  if (diff == 0) {
    answer = 0;
  } else if (diff > 0) {
    quotient = diff / 2;
    answer = quotient;
    if (diff % 2 == 1) {
      answer += 1;
    }
  } else {
    answer = -diff;
  }
  return answer;
}
}  // namespace
