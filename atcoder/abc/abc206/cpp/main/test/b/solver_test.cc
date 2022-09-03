#include "b/solver.h"

#include "gtest/gtest.h"

TEST(SolverBTest, Sample1) { EXPECT_EQ(solve(12), 5); }

TEST(SolverBTest, Sample2) { EXPECT_EQ(solve(100128), 447); }
