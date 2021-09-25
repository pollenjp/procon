#include <glog/logging.h>

#include <algorithm>
#include <cmath>
#include <map>
#include <vector>

namespace arc127 {

const int64_t kBaseNumber(10);
// 1000000000000000

int64_t solve(const int64_t n) {
  int32_t num_of_digits(0);
  num_of_digits = static_cast<int32_t>(std::log10(static_cast<long double>(n))) + 1;
  LOG(INFO) << "num_of_digits: " << num_of_digits;

  std::map<int64_t, int64_t> ones_cnt_map;

  for (int32_t i = 0; i < num_of_digits; i++) {
    int64_t ones(1);
    for (int32_t j = 0; j < i; j++) {
      ones += static_cast<int64_t>(std::pow(static_cast<long double>(kBaseNumber), static_cast<long double>(j + 1)));
    }
    LOG(INFO) << "loop: " << i << " - ones(" << ones << ")";

    for (int32_t j = 0; j < num_of_digits - i; j++) {
      // 11...1100...00 以上 11...1200...00 未満の数
      // or (min)
      // 11...1100...00 以上 N 以下の数
      // "未満" と "以下" に注意
      auto tmp = static_cast<int64_t>(std::pow(static_cast<long double>(kBaseNumber), static_cast<long double>(j)));
      if (n >= ones * tmp) {
        int64_t cnt = std::min(tmp, n + 1 - ones * tmp);
        LOG(INFO) << "(cnt: " << cnt << "), (tmp: " << tmp << "), (ones: " << ones << ")";
        ones_cnt_map[ones] += cnt;
      }
    }
  }

  int64_t sum(0);
  for (auto const& cnt_pair : ones_cnt_map) {
    LOG(INFO) << "cnt_pair = (" << cnt_pair.first << ", " << cnt_pair.second << ")";
    sum += cnt_pair.second;
  }

  return sum;
}
}  // namespace arc127
