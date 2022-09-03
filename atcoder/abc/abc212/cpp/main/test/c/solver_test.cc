#include "c/solver.h"

#include "gtest/gtest.h"

TEST(SolverCTest, Sample1) {
  std::vector<int> a({1, 6}), b({4, 9});
  EXPECT_EQ(solve(a, b), 2);
}

TEST(SolverCTest, Sample2) {
  std::vector<int> a({10}), b({10});
  EXPECT_EQ(solve(a, b), 0);
}

TEST(SolverCTest, Sample3) {
  std::vector<int> a({82, 76, 82, 82, 71, 70}),
      b({17, 39, 67, 2, 45, 35, 22, 24});
  EXPECT_EQ(solve(a, b), 3);
}
