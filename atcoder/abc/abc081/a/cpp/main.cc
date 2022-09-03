#include <bits/stdc++.h>

int main() {
  int32_t num_n(0);
  std::cin >> num_n;

  std::vector<int32_t> a_list(num_n);

  int32_t num_x(0);
  for (auto &val : a_list) {
    std::cin >> val;
    num_x |= val;
  }

  int32_t cnt(0);
  int32_t left(0);
  while (num_x != 0) {
    left = num_x % 2;
    if (left == 1) {
      break;
    }
    num_x /= 2;
    cnt++;
  }

  std::cout << cnt << std::endl;

  return 0;
}
