#include <glog/logging.h>

#include <numeric>
#include <set>
#include <string>
#include <utility>
#include <vector>

namespace atcoder {

namespace abc227 {

namespace c {

int64_t solve(int64_t num_n) {
  int64_t cnt(0);
  for (int64_t a = 1; a * a * a <= num_n; a++) {
    for (int64_t b = a; a * b * b <= num_n; b++) {
      int64_t tmp = num_n / (a * b) - b + 1;
      cnt += tmp;
    }
  }
  return cnt;
}

}  // namespace c

}  // namespace abc227

}  // namespace atcoder
