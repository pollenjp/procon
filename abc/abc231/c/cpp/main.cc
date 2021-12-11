#include <algorithm>
#include <iostream>
#include <map>
#include <memory>
#include <set>
#include <vector>

int main() {
  int32_t num_n(0);
  int32_t num_q(0);
  std::cin >> num_n >> num_q;

  std::vector<int32_t> a_list(num_n);

  for (int32_t i(0); i < num_n; ++i) {
    std::cin >> a_list[i];
  }
  std::sort(a_list.begin(), a_list.end());

  int32_t cnt(num_n);
  std::map<int32_t, int32_t> a_map;
  for (int32_t i(0); i < num_n; ++i) {
    if (a_map[a_list[i]] != 0)  // 身長 : i人目
    {
      continue;
    }
    a_map[a_list[i]] = i + 1;  // 身長 : i人目
  }

  std::vector<int32_t> b_list(num_q);
  for (int32_t i(0); i < num_q; ++i) {
    int32_t q(0);
    std::cin >> q;

    auto it = a_map.lower_bound(q);
    if (it == a_map.end()) {
      b_list[i] = 0;
    } else {
      b_list[i] = 1 + num_n - it->second;
    }
    // std::cout << num_n - it->second << std::endl;
  }

  for (auto &b : b_list) {
    std::cout << b << std::endl;
  }

  return 0;
}
