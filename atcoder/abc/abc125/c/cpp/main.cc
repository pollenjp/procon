#include <algorithm>
#include <atcoder/all>
#include <iostream>
#include <vector>

using std::cin;
using std::cout;
using std::endl;

int64_t gcd(int64_t a, int64_t b) {
  if (b == 0) {
    return a;
  }
  return gcd(b, a % b);
}

int64_t op(int64_t a, int64_t b) { return gcd(a, b); }

int64_t e() { return 0; }

int main() {
  int64_t in_n(0);
  cin >> in_n;

  std::vector<int64_t> in_a(in_n);
  for (auto &val : in_a) {
    cin >> val;
  }

  atcoder::segtree<int64_t, op, e> seg(in_a);
  std::vector<int64_t> gcd_vec(in_n);

  for (int32_t i(0); i < in_n; i++) {
    seg.set(i, in_a.at((i + 1) % in_n));
    gcd_vec.at(i) = seg.all_prod();
    seg.set(i, in_a.at(i));
  }

  cout << *std::max_element(begin(gcd_vec), end(gcd_vec)) << endl;
}
