// #include <glog/logging.h>

#include <cstdint>
#include <numeric>
#include <vector>

namespace atcoder {

namespace edpc {

namespace e {

/**
 * @brief dp for knapsack problem
 *
 * @param num_n
 * @param num_w
 * @param w_vec
 * @param v_vec
 * @return int64_t
 *
 * @details dp[i][j]: i番目までの品物を選んだときで価値がjになるときの重さの最小値
 */
int64_t solve(int32_t num_n, int32_t num_w, std::vector<int32_t> &w_vec, std::vector<int32_t> &v_vec) {
  auto max_sum_value = std::accumulate(v_vec.begin(), v_vec.end(), static_cast<int64_t>(0));
  auto max_sum_weight = std::accumulate(w_vec.begin(), w_vec.end(), static_cast<int64_t>(0));
  LOG(INFO) << "max_sum_value: " << max_sum_value;
  LOG(INFO) << "max_sum_weight: " << max_sum_weight;
  std::vector<std::vector<int64_t>> dp(num_n, std::vector<int64_t>(max_sum_value + 1, max_sum_weight));

  // init
  dp[0][0] = 0;
  dp[0][v_vec[0]] = w_vec[0];

  for (int32_t i = 1; i < num_n; i++) {
    for (int64_t v = 0; v <= max_sum_value; v++) {
      int64_t prev_v = v - static_cast<int64_t>(v_vec[i]);
      if (prev_v >= 0) {
        dp[i][v] = std::min(dp[i][v], dp[i - 1][prev_v] + w_vec[i]);
      }
      dp[i][v] = std::min(dp[i - 1][v], dp[i][v]);
      LOG(INFO) << "dp[" << i << "][" << v << "]: " << dp[i][v] << " (" << v_vec[i] << "," << w_vec[i] << ")";
    }
  }

  for (int64_t v = max_sum_value; v >= 0; v--) {
    LOG(INFO) << "dp[" << num_n - 1 << "][" << v << "] : " << dp[num_n - 1][v];
    if (dp[num_n - 1][v] <= num_w) {
      return v;
    }
  }

  return 0;
}

}  // namespace e

}  // namespace edpc

}  // namespace atcoder
