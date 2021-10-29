#include <glog/logging.h>

#include <algorithm>
#include <vector>

namespace atcoder {

namespace edpc {

namespace d {

/**
 * @brief dp for knapsack problem
 *
 * @param num_n
 * @param num_w
 * @param w_vec
 * @param v_vec
 * @return int64_t
 *
 * @details dp[i][j]: i番目までの品物を選んだときで重さがj以下であるときの最大価値
 */
int64_t solve(int32_t num_n, int32_t num_w, std::vector<int32_t> &w_vec, std::vector<int32_t> &v_vec) {
  std::vector<std::vector<int64_t>> dp(num_n, std::vector<int64_t>(num_w + 1, 0));

  // init
  for (int32_t w = 0; w <= num_w; w++) {
    if (w_vec[0] > w) {
      dp[0][w] = 0;
    } else {
      dp[0][w] = static_cast<int64_t>(v_vec[0]);
    }
    LOG(INFO) << "dp[0][" << w << "]: " << dp[0][w];
  }

  // int64_t reward_max(0);
  for (int32_t i = 1; i < num_n; i++) {
    for (int32_t w = 0; w <= num_w; w++) {
      if (w_vec[i] > w) {
        dp[i][w] = dp[i - 1][w];
      } else {
        dp[i][w] = std::max(dp[i - 1][w], static_cast<int64_t>(v_vec[i]) + dp[i - 1][w - w_vec[i]]);
      }
      LOG(INFO) << "dp[" << i << "][" << w << "]: " << dp[i][w] << "(" << v_vec[i] << "," << w_vec[i] << ")";
    }
  }

  return dp[num_n - 1][num_w];
}

}  // namespace d

}  // namespace edpc

}  // namespace atcoder
