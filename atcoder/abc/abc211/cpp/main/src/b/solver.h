#include <map>
#include <string>
#include <vector>

namespace {

bool solve(const std::vector<std::string> &s_vec) {
  std::map<std::string, int> s_counter;

  for (auto itr = s_vec.begin(); itr != s_vec.end(); ++itr) {
    s_counter[*itr]++;
    if (s_counter[*itr] > 1) {
      return false;
    }
  }
  return true;
}
}  // namespace
