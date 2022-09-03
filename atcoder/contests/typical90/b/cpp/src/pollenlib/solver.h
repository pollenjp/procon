#include <glog/logging.h>

#include <algorithm>
#include <string>
#include <vector>

namespace atcoder {

namespace typical90 {

namespace b {

/**
 * @brief
 *
 * @param s
 * @return true
 * @return false
 * @details 常に '(' と ')' の数が同じであることが前提.
 */
bool IsValid(const std::string& s) {
  int32_t pre_parenthesis_count(0);
  for (char c : s) {
    if (c == '(') {
      pre_parenthesis_count++;
    } else {
      pre_parenthesis_count--;
      if (pre_parenthesis_count < 0) {
        return false;
      }
    }
  }
  return true;
}

std::vector<std::string> solve(int32_t num_n) {
  if (num_n % 2 == 1) {
    return std::vector<std::string>(0);
  }

  std::vector<std::string> target_list;

  std::string s(num_n, '(');

  for (int32_t i = num_n / 2; i < num_n; i++) {
    s[i] = ')';
  }

  do {
    if (IsValid(s)) {
      target_list.push_back(std::string(s));
    }
  } while (std::next_permutation(s.begin(), s.end()));

  return target_list;
}

}  // namespace b

}  // namespace typical90

}  // namespace atcoder
