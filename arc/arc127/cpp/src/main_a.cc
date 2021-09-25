#include <iostream>

#include "a/solver.h"

int main(int argc, char* argv[]) {
  google::InitGoogleLogging(argv[0]);
  gflags::ParseCommandLineFlags(&argc, &argv, true);

  int64_t n(0);  // 4 <= n <= 2,000

  std::cin >> n;

  LOG(INFO) << "Input n = " << n;

  std::cout << arc127::solve(n) << std::endl;

  return EXIT_SUCCESS;
}
