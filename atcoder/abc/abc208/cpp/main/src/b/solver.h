
#include <cmath>
#include <iostream>
#include <vector>

namespace {

int solve(const int p) {
  int left = p;
  int max_n = 10;
  std::vector<int> stock(max_n, 0);
  int tmp = 1;
  for (int i = 0; i < max_n; i++) {
    tmp *= (i + 1);
    stock[i] = tmp;
  }

  int n_coin = 0;
  for (int i = 0; i < max_n; i++) {
    auto n = max_n - 1 - i;
    auto val = stock[n];
    while (true) {
      if (left >= val) {
        n_coin++;
        left -= val;
      } else {
        break;
      }
    }
    if (left == 0) {
      break;
    }
  }
  return n_coin;
}
}  // namespace
