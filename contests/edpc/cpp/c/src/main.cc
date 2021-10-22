#include <iostream>

#include "pollenlib/solver.h"

int main(int argc, char* argv[]) {
  google::InitGoogleLogging(argv[0]);
  gflags::ParseCommandLineFlags(&argc, &argv, true);

  int32_t num_n(0);  // 1 <= num_n <= 10^5

  std::cin >> num_n;

  LOG(INFO) << "Input num_n = " << num_n;

  std::vector<int32_t> a_vec(num_n, 0), b_vec(num_n, 0), c_vec(num_n, 0);  // 1 <= a_i, b_i, c_i <= 10^4

  for (int32_t i = 0; i < num_n; i++) {
    std::cin >> a_vec[i] >> b_vec[i] >> c_vec[i];
  }

  std::cout << atcoder::edpc::c::solve(num_n, a_vec, b_vec, c_vec) << std::endl;

  return EXIT_SUCCESS;
}
