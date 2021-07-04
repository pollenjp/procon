#include "b/solver.h"

#include "gtest/gtest.h"

TEST(SolverBTest, Sample1) { EXPECT_EQ(solve(5, 2, 3, 2), 2); }

TEST(SolverBTest, Sample2) { EXPECT_EQ(solve(6, 9, 2, 3), -1); }
