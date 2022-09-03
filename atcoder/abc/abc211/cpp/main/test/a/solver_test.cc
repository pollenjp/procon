#include "a/solver.h"

#include "gtest/gtest.h"

#define EPSILON 1e-5

TEST(SolverATest, Sample1) { EXPECT_NEAR(solve(130, 100), 110, EPSILON); }

TEST(SolverATest, Sample2) {
  EXPECT_NEAR(solve(300, 50), 133.3333333, EPSILON);
}

TEST(SolverATest, Sample3) { EXPECT_NEAR(solve(123, 123), 123, EPSILON); }
