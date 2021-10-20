
#include <iostream>

namespace {

int solve(int N) {
  int n_date = 1;
  long long tmp = 0;
  long long tmp2 = 0;

  for (n_date = 1; n_date <= N; n_date++) {
    tmp += static_cast<long long>(n_date);
    if (tmp >= N) {
      break;
    }
  }
  return n_date;
}
}  // namespace
