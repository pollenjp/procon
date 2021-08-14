#include "a/solver.h"

#include "gtest/gtest.h"

#define EPSILON 1e-5

TEST(SolverATest, Sample1) { EXPECT_EQ(solve(3, 6), 5); }

TEST(SolverATest, Sample2) { EXPECT_EQ(solve(10, 12), 6); }
