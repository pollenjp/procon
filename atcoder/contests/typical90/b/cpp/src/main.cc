#include <iostream>

#include "pollenlib/solver.h"

int main(int argc, char* argv[]) {
  google::InitGoogleLogging(argv[0]);
  gflags::ParseCommandLineFlags(&argc, &argv, true);

  int32_t num_n(0);  // 1 <= num_n <=10^2

  std::cin >> num_n;

  LOG(INFO) << "Input num_n = " << num_n;

  auto target_list = atcoder::typical90::b::solve(num_n);
  for (auto target : target_list) {
    std::cout << target << std::endl;
  }

  return EXIT_SUCCESS;
}
