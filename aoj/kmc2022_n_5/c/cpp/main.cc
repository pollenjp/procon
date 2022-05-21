#include <algorithm>
#include <iostream>
#include <vector>

int main() {
  int32_t n{};
  std::cin >> n;
  std::vector<int32_t> a_vec(n, 0);
  for (auto &x : a_vec) {
    std::cin >> x;
  }

  int32_t max_a = *std::max_element(a_vec.begin(), a_vec.end());

  // Eratosthenes
  std::vector<bool> is_prime(max_a + 1, true);
  for (int32_t i = 2; i <= max_a; i++) {
    if (!is_prime[i]) {
      continue;
    }

    for (int32_t k = i * 2; k <= max_a; k += i) {
      is_prime[k] = false;
    }
  }

  int32_t cnt{};
  for (auto &x : a_vec) {
    if (is_prime[x]) {
      cnt++;
    }
  }
  std::cout << cnt << std::endl;
}
