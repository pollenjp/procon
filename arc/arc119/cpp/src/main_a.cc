#include <cmath>
#include <iostream>
#include <limits>

long long solve(const long long n) {
  long long a0, b0(0), c0(0);
  long double b1 = std::log2(static_cast<long double>(n));
  long long b2 = static_cast<long double>(std::floor(b1));
  long long a_b_c_min = -1;
  for (auto b = b2; b >= 0; b--) {
    long long num_2_pow_b = std::pow(2, b);
    a0 = n / num_2_pow_b;
    c0 = n - a0 * num_2_pow_b;
    long long a_b_c = a0 + b + c0;
    if (a_b_c_min < 0 || a_b_c <= a_b_c_min) {
      a_b_c_min = a_b_c;
    }
  }
  return a_b_c_min;
}

int main() {
  long long N;
  std::cin >> N;
  std::cout << solve(N) << std::endl;
  return EXIT_SUCCESS;
}
