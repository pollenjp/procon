#include <pollenlib/solver.h>

#include <iostream>
#include <vector>

int main(int argc, char* argv[]) {
  google::InitGoogleLogging(argv[0]);
  gflags::ParseCommandLineFlags(&argc, &argv, true);

  int64_t num_n(0);  // 1 <= num_n <= 10^5

  std::cin >> num_n;

  std::cout << atcoder::abc227::c::solve(num_n) << std::endl;

  return EXIT_SUCCESS;
}
