#include <iostream>

constexpr const int64_t kP = 1000000007;  // prime number

int64_t pow_ans(int64_t x, int32_t index) {
  int64_t res = 1;
  while (index > 0) {
    if (index & 1) {
      res *= x;
      res %= kP;
    }
    x *= x;
    x %= kP;
    index >>= 1;
  }
  return res;
}

int main() {
  int64_t m{};
  int64_t n{};
  std::cin >> m >> n;

  std::cout << pow_ans(m, n) << std::endl;
}
