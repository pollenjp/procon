#include "a/solver.h"

#include "gtest/gtest.h"

TEST(SolverATest, Sample1) {
  int a = 2, b = 5, c = 2;
  EXPECT_EQ(solve(a, b, c), 5);
}

TEST(SolverATest, Sample2) {
  int a = 4, b = 5, c = 6;
  EXPECT_EQ(solve(a, b, c), 0);
}

TEST(SolverATest, Sample3) {
  int a = 1, b = 1, c = 1;
  EXPECT_EQ(solve(a, b, c), 1);
}
