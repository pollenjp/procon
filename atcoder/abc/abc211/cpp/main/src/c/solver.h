#include <string>
#include <vector>

namespace {

std::string TARGET("chokudai");
unsigned int MOD = static_cast<unsigned int>(1e9) + 7;

unsigned int solve(const std::string &s) {
  //! dp[i][j]
  //! := s の i 文字目までを使って, TARGET の j 文字目までを構成する場合の数
  std::vector<std::vector<unsigned int>> dp(
      s.size() + 1, std::vector<unsigned int>(TARGET.size() + 1, 0));

  for (unsigned int i = 0; i <= s.size(); ++i) {
    for (unsigned int j = 0; j <= TARGET.size(); ++j) {
      if (j == 0) {
        dp[i][j] = 1;
      } else if (i == 0) {
        dp[i][j] = 0;
      } else if (s[i - 1] != TARGET[j - 1]) {
        dp[i][j] = dp[i - 1][j];
      } else {
        dp[i][j] = (dp[i - 1][j] + dp[i - 1][j - 1]) % MOD;
      }
    }
  }

  return dp[s.size()][TARGET.size()] % MOD;
}
}  // namespace
