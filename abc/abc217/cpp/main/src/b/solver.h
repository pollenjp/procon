#include <algorithm>
#include <map>
#include <stdexcept>
#include <string>
#include <vector>

namespace abc217 {

std::string solve(const std::string &s1, const std::string &s2, const std::string &s3) {
  std::map<std::string, int> mp;
  mp[s1]++;
  mp[s2]++;
  mp[s3]++;

  std::vector<std::string> s_vec({"ABC", "ARC", "AGC", "AHC"});
  for (auto &s : s_vec) {
    if (mp[s] == 0) {
      return s;
    }
  }

  throw std::runtime_error("not matched");
}
}  // namespace abc217
