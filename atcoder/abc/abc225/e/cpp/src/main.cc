#include <iostream>
#include <utility>

#include "pollenlib/solver.h"

int main(int argc, char* argv[]) {
  google::InitGoogleLogging(argv[0]);
  gflags::ParseCommandLineFlags(&argc, &argv, true);

  int32_t num_n(0);  // 1 <= num_n <= 10^5

  std::cin >> num_n;

  LOG(INFO) << "Input num_n = " << num_n;

  std::vector<std::pair<int64_t, int64_t>> xy_list(num_n, {0, 0});

  for (auto& xy_pair : xy_list) {
    std::cin >> xy_pair.first >> xy_pair.second;
    LOG(INFO) << xy_pair.first << ", " << xy_pair.second;
  }

  std::cout << atcoder::abc225::e::solve(xy_list) << std::endl;

  return EXIT_SUCCESS;
}
