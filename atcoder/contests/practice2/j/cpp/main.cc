#include <atcoder/all>
#include <iostream>
#include <vector>

using std::cin;
using std::cout;
using std::endl;

int op(int a, int b) { return std::max(a, b); }

int e() { return -1; }

int target;

bool f(int v) { return v < target; }

int main() {
  int32_t in_n(0);
  int32_t in_q(0);
  cin >> in_n >> in_q;

  std::vector<int32_t> in_a(in_n);
  for (auto &val : in_a) {
    cin >> val;
  }

  atcoder::segtree<int32_t, op, e> seg(in_a);

  for (int32_t i = 0; i < in_q; ++i) {
    int32_t in_q1(0);
    int32_t in_q2(0);
    int32_t in_q3(0);
    cin >> in_q1 >> in_q2 >> in_q3;
    switch (in_q1) {
      case 1:
        seg.set(--in_q2, in_q3);
        break;
      case 2:
        cout << seg.prod(--in_q2, in_q3) << endl;
        break;
      case 3:
        target = in_q3;
        cout << (seg.max_right<f>(--in_q2) + 1) << endl;
        break;
    }
  }
}
