#include <atcoder/all>
#include <iostream>
#include <vector>

using std::cin;
using std::cout;
using std::endl;

int64_t op(int64_t a, int64_t b) { return a ^ b; }

int64_t e() { return 0; }

int main() {
  int64_t in_n(0);
  int64_t in_q(0);
  cin >> in_n >> in_q;

  std::vector<int64_t> in_a(in_n);
  for (auto &val : in_a) {
    cin >> val;
  }

  atcoder::segtree<int64_t, op, e> seg(in_a);

  for (int64_t i = 0; i < in_q; ++i) {
    int64_t in_q1(0);
    int64_t in_q2(0);
    int64_t in_q3(0);
    cin >> in_q1 >> in_q2 >> in_q3;
    switch (in_q1) {
      case 1:
        in_q2--;
        seg.set(in_q2, seg.get(in_q2) ^ in_q3);
        break;
      case 2:
        cout << seg.prod(--in_q2, in_q3) << endl;
        break;
    }
  }
}
