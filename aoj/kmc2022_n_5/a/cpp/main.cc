#include <iostream>

int64_t gcd(int64_t a, int64_t b) {
  if (b == 0) {
    return a;
  }
  return gcd(b, a % b);
}

int main() {
  int a{};
  int b{};
  while (std::cin >> a >> b) {
    int64_t g = gcd(a, b);
    int64_t lcm = (a / g) * b;
    std::cout << g << " " << lcm << std::endl;
  }
  return 0;
}
