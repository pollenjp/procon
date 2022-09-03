#include "a/solver.h"

#include "gtest/gtest.h"

TEST(SolverATest, Sample1) {
  int a = 1, b = 4, c = 3;
  EXPECT_EQ(solve(a, b, c), 13);
}

TEST(SolverATest, Sample2) {
  int a = 5, b = 6, c = 4;
  EXPECT_EQ(solve(a, b, c), 6);
}
