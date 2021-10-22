#include <glog/logging.h>

#include <algorithm>
#include <vector>

namespace atcoder {

namespace edpc {

namespace c {

int32_t solve(int32_t num_n, std::vector<int32_t> &a_vec, std::vector<int32_t> &b_vec, std::vector<int32_t> &c_vec) {
  int32_t num_actions(3);
  std::vector<std::vector<int32_t>> action_reward_vec(num_n, std::vector<int32_t>(num_actions, 0));
  for (std::size_t i = 0; i < action_reward_vec.size(); i++) {
    action_reward_vec[i][0] = a_vec[i];
    action_reward_vec[i][1] = b_vec[i];
    action_reward_vec[i][2] = c_vec[i];
  }
  // i : action
  // 0 : a
  // 1 : b
  // 2 : c
  std::vector<std::vector<int32_t>> reward_dp(num_n, std::vector<int32_t>(num_actions, 0));

  // first element
  reward_dp[0][0] = a_vec[0];
  reward_dp[0][1] = b_vec[0];
  reward_dp[0][2] = c_vec[0];

  for (std::size_t i = 1; i < reward_dp.size(); i++) {
    for (int32_t k = 0; k < num_actions; k++) {
      reward_dp[i][k] = action_reward_vec[i][k] +
                        std::max(reward_dp[i - 1][(k + 1) % num_actions], reward_dp[i - 1][(k + 2) % num_actions]);
    }
  }

  return std::max(reward_dp[num_n - 1][0], std::max(reward_dp[num_n - 1][1], reward_dp[num_n - 1][2]));
}

}  // namespace c

}  // namespace edpc

}  // namespace atcoder
