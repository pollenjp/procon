#include <iostream>
#include <set>
#include <vector>

int main(int argc, char* argv[]) {
  int32_t num_n(0);  // 1 <= num_n <= 10^5

  std::cin >> num_n;

  std::set<std::vector<int32_t>> l_list_set;

  for (int32_t i = 0; i < num_n; ++i) {
    int32_t num_l(0);
    std::cin >> num_l;
    std::vector<int32_t> l_list(num_l);
    for (int32_t j = 0; j < num_l; ++j) {
      std::cin >> l_list[j];
    }

    l_list_set.insert(l_list);
  }

  std::cout << l_list_set.size() << std::endl;

  return EXIT_SUCCESS;
}
