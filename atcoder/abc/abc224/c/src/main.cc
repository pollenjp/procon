#include <iostream>

#include "pollenlib/solver.h"

int main(int argc, char* argv[]) {
  google::InitGoogleLogging(argv[0]);
  gflags::ParseCommandLineFlags(&argc, &argv, true);

  int32_t num_n(0);  // 1 <= num_n <= 10^5

  std::cin >> num_n;

  LOG(INFO) << "Input num_n = " << num_n;

  std::vector<std::vector<int64_t>> xy_vec(num_n, std::vector<int64_t>(2, 0));

  for (int32_t i = 0; i < num_n; i++) {
    std::cin >> xy_vec[i][0] >> xy_vec[i][1];
  }

  std::cout << atcoder::abc224::c::solve(xy_vec) << std::endl;

  return EXIT_SUCCESS;
}
