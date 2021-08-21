#include <algorithm>
#include <iostream>
#include <map>
#include <string>
#include <vector>

namespace {

std::vector<int> Eratosthenes(const int n) {
  std::vector<bool> is_prime(n + 1);
  for (int i = 0; i <= n; i++) {
    is_prime[i] = true;
  }
  std::vector<int> primes;
  for (int i = 2; i <= n; i++) {
    if (is_prime[i]) {
      for (int j = 2 * i; j <= n; j += i) {
        is_prime[j] = false;
      }
      primes.emplace_back(i);
    }
  }
  return primes;
}

std::vector<int> solve(const int n, const int m, std::vector<int> vec) {
  std::vector<int> valid_numbers({});
  std::vector<int> primes_below_m = Eratosthenes(m);

  if (n >= 1) {
    valid_numbers.push_back(1);
  }
  for (std::size_t i = 0; i < primes_below_m.size(); ++i) {
    bool is_valid = true;
    for (std::size_t j = 0; j < vec.size(); ++j) {
      if (vec[j] % primes_below_m[i] == 0) {
        is_valid = false;
        break;
      }
    }

    if (is_valid) {
      // std::cout << primes_below_m[i] << std::endl;
      valid_numbers.push_back(primes_below_m[i]);
    }
  }

  return valid_numbers;
}

}  // namespace
