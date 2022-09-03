#include "c/solver.h"

#include <vector>

#include "gtest/gtest.h"

TEST(SolverCTest, Sample1) {
  std::vector<int> vec({2, 7});
  std::vector<long long> correct_out({4, 3});
  EXPECT_EQ(solve(2, 7, vec), correct_out);
}

TEST(SolverCTest, CustomSample1) {
  std::vector<int> vec({1});
  std::vector<long long> correct_out({1000000000000000000});
  EXPECT_EQ(solve(1, 1000000000000000000 /* = 10^18 */, vec), correct_out);
}

TEST(SolverCTest, CustomSample2) {
  std::vector<int> vec({3, 1, 2});
  std::vector<long long> correct_out({1, 1, 1});
  EXPECT_EQ(solve(3, 3, vec), correct_out);
}

TEST(SolverCTest, CustomSample3) {
  int N = 2 * 100000;  // 2*10^5
  long long K = 3;
  std::vector<int> vec(N, 1);
  for (int i = 0; i < N; i++) {
    vec[i] = 10 * N - i;
  }
  std::vector<long long> correct_out(N, 0);
  for (int i = 0; i < 3; i++) {
    correct_out[N - 1 - i] = 1;
  }
  EXPECT_EQ(solve(N, K, vec), correct_out);
}

// TEST(SolverCTest, CustomSample3) {
//   int N = 2 * 100000;  // 2*10^5
//   long long K = 1000000000000000000;
//   std::vector<int> vec(N, 1);
//   for (int i = 0; i < N; i++) {
//     vec[i] = 10 * N - i;
//   }
//   std::vector<long long> correct_out({1, 1, 1});
//   EXPECT_EQ(solve(N, K /* = 10^18 */, vec), correct_out);
// }
