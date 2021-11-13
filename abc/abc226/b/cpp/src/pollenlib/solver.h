#include <glog/logging.h>

#include <numeric>
#include <set>
#include <string>
#include <utility>
#include <vector>

namespace atcoder {

namespace abc226 {

namespace b {

std::size_t solve(const std::vector<std::vector<int32_t>> &l_list_list) {
  std::set<std::vector<int32_t>> bin_tree;
  for (auto &l_list : l_list_list) {
    bin_tree.insert(l_list);
  }
  return bin_tree.size();
}

}  // namespace b

}  // namespace abc226

}  // namespace atcoder
