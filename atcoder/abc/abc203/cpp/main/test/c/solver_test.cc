#include "c/solver.h"

#include "gtest/gtest.h"

TEST(SolverCTest, Sample1) {
  long long N(2), K(3);
  std::vector<std::pair<long long, long long>> ab_vectors({{{2, 1}, {5, 10}}});
  EXPECT_EQ(solve(N, K, ab_vectors), 4);
}

TEST(SolverCTest, Sample2) {
  long long N(5), K(1000000000);
  std::vector<std::pair<long long, long long>> ab_vectors({{{1, 1000000000},
                                                            {2, 1000000000},
                                                            {3, 1000000000},
                                                            {4, 1000000000},
                                                            {5, 1000000000}}});
  EXPECT_EQ(solve(N, K, ab_vectors), 6000000000);
}

TEST(SolverCTest, Sample3) {
  long long N(3), K(2);
  std::vector<std::pair<long long, long long>> ab_vectors(
      {{{5, 5}, {2, 1}, {2, 2}}});
  EXPECT_EQ(solve(N, K, ab_vectors), 10);
}

// TODO: max
// TEST(SolverCTest, MaxSample) {
//   long long N(3), K(2);
//   std::vector<std::pair<long long, long long>> ab_vectors(
//       {{{5, 5}, {2, 1}, {2, 2}}});
//   EXPECT_EQ(solve(N, K, ab_vectors), 10);
// }
