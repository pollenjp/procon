#include <iostream>

#include "pollenlib/solver.h"

int main(int argc, char* argv[]) {
  google::InitGoogleLogging(argv[0]);
  gflags::ParseCommandLineFlags(&argc, &argv, true);

  int32_t num_n(0);  // 1 <= num_n <=10^2
  int32_t num_w(0);  // 1 <= num_w <= 10^5

  std::cin >> num_n >> num_w;

  LOG(INFO) << "Input num_n = " << num_n << ", num_w = " << num_w;

  std::vector<int32_t> w_vec(num_n, 0), v_vec(num_n, 0);

  for (int32_t i = 0; i < num_n; i++) {
    std::cin >> w_vec[i] >> v_vec[i];
  }

  std::cout << atcoder::edpc::d::solve(num_n, num_w, w_vec, v_vec) << std::endl;

  return EXIT_SUCCESS;
}
