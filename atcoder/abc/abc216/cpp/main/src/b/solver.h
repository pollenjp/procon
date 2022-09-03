#include <algorithm>
#include <vector>
#include <string>

namespace {

bool solve(const std::vector<std::pair<std::string, std::string>> &s_vec) {
  for (std::size_t i = 0; i < s_vec.size() - 1; i++) {
    for (std::size_t j = i + 1; j < s_vec.size(); j++) {
      if (s_vec[i].first == s_vec[j].first && s_vec[i].second == s_vec[j].second) {
        return true;
      }
    }
  }
  return false;

}
}  // namespace
