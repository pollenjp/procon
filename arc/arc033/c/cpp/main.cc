#include <atcoder/all>
#include <iostream>
#include <set>
#include <vector>

using std::cin;
using std::cout;
using std::endl;

int op(int a, int b) { return a + b; }

int e() { return 0; }

constexpr int32_t kMaxX(200000);

int main() {
  int64_t in_q(0);
  cin >> in_q;

  atcoder::segtree<int32_t, op, e> seg(kMaxX + 1);

  for (int64_t i = 0; i < in_q; ++i) {
    int64_t in_q1(0);
    int64_t in_q2(0);
    cin >> in_q1 >> in_q2;
    // cout << in_q1 << " : " << in_q2 << endl;
    switch (in_q1) {
      case 1:
        seg.set(in_q2, 1);
        break;
      case 2:
        auto left = 0;
        auto right = kMaxX;
        while (right - left > 1) {
          int mid = left + (right - left) / 2;

          if (seg.prod(0, mid + 1) >= in_q2) {
            right = mid;
          } else {
            left = mid;
          }
        }
        // cout << left << ":" << right << endl;
        cout << right << endl;
        seg.set(right, 0);
        break;
    }
  }
}
