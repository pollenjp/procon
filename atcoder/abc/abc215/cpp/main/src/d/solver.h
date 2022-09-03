#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

namespace {

/**
 * @brief 素因数分解
 * T : int-like
 */
template <typename T>
std::vector<std::pair<T, T>> PrimeFactorize(T n) {
  std::vector<std::pair<T, T>> ret;

  if (n <= 0) {
    throw std::invalid_argument("n must be positive");
  }

  for (T i = 2; i * i <= n; i++) {
    if (n % i != 0) {
      continue;
    }
    T squre_times(0);
    while (n % i == 0) {
      squre_times++;
      n /= i;
    }
    ret.push_back(std::make_pair(i, squre_times));
  }
  if (n != 1) {
    ret.push_back(std::make_pair(n, 1));
  }
  return ret;
}

std::vector<int> solve(const int n, const int m, const std::vector<int> &vec) {
  std::vector<int> valid_numbers;
  int max_num = *std::max_element(vec.begin(), vec.end());
  max_num = std::max(max_num, m);
  std::vector<bool> is_valid_num_vec(max_num + 1, true);

  for (auto &val : vec) {
    auto primes = PrimeFactorize(val);
    for (auto &prime_pair : primes) {
      if (!is_valid_num_vec[prime_pair.first]) {
        continue;
      }
      for (auto i = prime_pair.first; i <= max_num; i += prime_pair.first) {
        is_valid_num_vec[i] = false;
      }
    }
  }

  for (int i = 1; i <= m; i++) {
    if (is_valid_num_vec[i]) {
      valid_numbers.push_back(i);
    }
  }

  return valid_numbers;
}
}  // namespace
