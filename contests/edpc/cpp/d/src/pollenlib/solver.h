#include <glog/logging.h>

#include <algorithm>
#include <vector>

namespace atcoder {

namespace edpc {

namespace d {

int64_t solve(int32_t num_n, int32_t num_w, std::vector<int32_t> &w_vec, std::vector<int32_t> &v_vec) {
  std::vector<std::vector<int64_t>> dp(num_n, std::vector<int64_t>(num_w + 1, 0));
  for (int32_t w = 0; w <= num_w; w++) {
    if (w_vec[0] > w) {
      dp[0][w] = 0;
    } else {
      dp[0][w] = v_vec[0];
    }
    LOG(INFO) << "dp[0][" << w << "]: " << dp[0][w];
  }

  int64_t reward_max(0);
  for (int32_t i = 1; i < num_n; i++) {
    for (int32_t w = 0; w <= num_w; w++) {
      if (w_vec[i] > w) {
        dp[i][w] = dp[i - 1][w];
      } else {
        int64_t w2 = w - w_vec[i];
        dp[i][w] = v_vec[i] + dp[i - 1][w2];
      }
      LOG(INFO) << "dp[" << i << "][" << w << "]: " << dp[i][w] << "(" << v_vec[i] << "," << w_vec[i] << ")";
    }
    reward_max = std::max(reward_max, *std::max_element(dp[i].begin(), dp[i].end()));
    LOG(INFO) << i << "'s reward_max: " << reward_max;
  }

  return reward_max;
}

}  // namespace d

}  // namespace edpc

}  // namespace atcoder
