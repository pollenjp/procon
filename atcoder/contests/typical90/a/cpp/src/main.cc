#include <iostream>

#include "pollenlib/solver.h"

int main(int argc, char* argv[]) {
  google::InitGoogleLogging(argv[0]);
  gflags::ParseCommandLineFlags(&argc, &argv, true);

  int32_t num_n(0);  // 1 <= num_n <=10^5
  int32_t num_l(0);  // 1 <= num_w <= 10^9
  int32_t num_k(0);  // 1 <= num_w <= num_n

  std::cin >> num_n >> num_l >> num_k;

  LOG(INFO) << "Input num_n = " << num_n << "num_l = " << num_l << "num_k = " << num_k;

  std::vector<int32_t> a_list(num_n, 0);

  for (auto& a_val : a_list) {
    std::cin >> a_val;
  }

  std::cout << atcoder::typical90::a::solve(num_n, num_l, num_k, a_list) << std::endl;

  return EXIT_SUCCESS;
}
