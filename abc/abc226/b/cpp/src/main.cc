#include <pollenlib/solver.h>

#include <iostream>
#include <vector>

int main(int argc, char* argv[]) {
  google::InitGoogleLogging(argv[0]);
  gflags::ParseCommandLineFlags(&argc, &argv, true);

  int32_t num_n(0);  // 1 <= num_n <= 10^5

  std::cin >> num_n;

  std::vector<std::vector<int32_t>> l_list_list(num_n);

  for (int32_t i = 0; i < num_n; ++i) {
    int32_t num_l(0);
    std::cin >> num_l;
    std::vector<int32_t> l_list(num_l);
    for (int32_t j = 0; j < num_l; ++j) {
      std::cin >> l_list[j];
    }
    l_list_list[i] = std::move(l_list);
  }

  std::cout << atcoder::abc226::b::solve(l_list_list) << std::endl;

  return EXIT_SUCCESS;
}
