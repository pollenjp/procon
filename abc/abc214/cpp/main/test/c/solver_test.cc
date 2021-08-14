#include "c/solver.h"

#include "gtest/gtest.h"

TEST(SolverCTest, Sample1) {
  std::vector<int> a({4, 1, 5}), b({3, 10, 100}), ans({3, 7, 8});
  EXPECT_EQ(solve(a, b), ans);
}

TEST(SolverCTest, Sample2) {
  std::vector<int> a({100, 100, 100, 100}), b({1, 1, 1, 1}), ans({1, 1, 1, 1});
  EXPECT_EQ(solve(a, b), ans);
}

TEST(SolverCTest, Sample3) {
  std::vector<int> a({1, 2, 3, 4}), b({1, 2, 4, 7}), ans({1, 2, 4, 7});
  EXPECT_EQ(solve(a, b), ans);
}

TEST(SolverCTest, Sample4) {
  std::vector<int> a({84, 87, 78, 16, 94, 36, 87, 93}),
      b({50, 22, 63, 28, 91, 60, 64, 27}),
      ans({50, 22, 63, 28, 44, 60, 64, 27});
  EXPECT_EQ(solve(a, b), ans);
}
