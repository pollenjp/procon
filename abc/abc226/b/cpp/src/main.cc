#include <iostream>
#include <numeric>
#include <utility>
#include <vector>

int main(int argc, char* argv[]) {
  int32_t num_n(0);  // 1 <= num_n <= 10^5

  std::cin >> num_n;

  std::vector<std::vector<int32_t>> l_list_list(num_n);
  std::size_t l_list_idx(0);

  for (int32_t i = 0; i < num_n; ++i) {
    int32_t num_l(0);
    std::cin >> num_l;
    std::vector<int32_t> l_list(num_l);
    for (int32_t j = 0; j < num_l; ++j) {
      std::cin >> l_list[j];
    }

    bool is_same(false);
    for (std::size_t j = 0; j < l_list_idx; ++j) {
      if (l_list_list[j].size() == l_list.size()) {
        is_same = true;
        for (std::size_t k = 0; k < l_list_list[j].size(); ++k) {
          if (l_list_list[j][k] != l_list[k]) {
            is_same = false;
            break;
          }
        }
      }
      if (is_same) {
        break;
      }
    }
    // std::cout << is_same << std::endl;
    if (!is_same) {
      l_list_list[l_list_idx] = l_list;
      l_list_idx++;
    }
  }

  std::cout << l_list_idx << std::endl;

  return EXIT_SUCCESS;
}
