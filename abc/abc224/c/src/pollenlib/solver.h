#include <glog/logging.h>

#include <algorithm>
#include <vector>

namespace atcoder {

namespace abc224 {

namespace c {

bool is_valid_triangle(std::vector<int64_t> const &xy1, std::vector<int64_t> const &xy2,
                       std::vector<int64_t> const &xy3) {
  int64_t ab_vec_x = xy2[0] - xy1[0];
  int64_t ab_vec_y = xy2[1] - xy1[1];

  int64_t ac_vec_x = xy3[0] - xy1[0];
  int64_t ac_vec_y = xy3[1] - xy1[1];

  int64_t outer_prod_z = ab_vec_x * ac_vec_y - ab_vec_y * ac_vec_x;
  return outer_prod_z != 0;
}

int64_t solve(std::vector<std::vector<int64_t>> const &xy_vec) {
  int64_t cnt(0);

  for (std::size_t idx1 = 0; idx1 < xy_vec.size() - 2; idx1++) {
    for (std::size_t idx2 = idx1 + 1; idx2 < xy_vec.size() - 1; idx2++) {
      for (std::size_t idx3 = idx2 + 1; idx3 < xy_vec.size(); idx3++) {
        if (is_valid_triangle(xy_vec[idx1], xy_vec[idx2], xy_vec[idx3])) {
          cnt++;
          LOG(INFO) << "cnt = " << cnt;
        }
      }
    }
  }

  return cnt;
}

}  // namespace c

}  // namespace abc224

}  // namespace atcoder
