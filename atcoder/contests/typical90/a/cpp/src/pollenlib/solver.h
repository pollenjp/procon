#include <glog/logging.h>

#include <algorithm>
#include <vector>

namespace atcoder {

namespace typical90 {

namespace a {

bool check(const int32_t cut_length_min, const int32_t min_cnt, const int32_t all_length,
           const std::vector<int32_t> &a_list) {
  int32_t cut_cnt(0);
  int32_t latest_cut_location(0);

  for (std::size_t i(0); i < a_list.size(); i++) {
    LOG(INFO) << "latest_cut_location=" << latest_cut_location;
    if (a_list[i] - latest_cut_location >= cut_length_min && all_length - a_list[i] >= cut_length_min) {
      cut_cnt++;
      latest_cut_location = a_list[i];
    }
  }

  return cut_cnt >= min_cnt;
}

int32_t solve(const int32_t num_n, const int32_t num_l, const int32_t num_k, const std::vector<int32_t> &a_list) {
  // めぐる式二分探索
  int32_t left = -1;
  int32_t right = num_l + 1;
  while (right - left > 1) {
    int32_t mid = left + (right - left) / 2;
    LOG(INFO) << "left: " << left << ", right: " << right << ", mid: " << mid;
    if (check(mid, num_k, num_l, a_list)) {
      left = mid;
    } else {
      right = mid;
    }
  }

  return left;
}

}  // namespace a

}  // namespace typical90

}  // namespace atcoder
