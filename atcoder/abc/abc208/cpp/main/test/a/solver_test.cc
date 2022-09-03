#include "a/solver.h"

#include "gtest/gtest.h"

TEST(SolverATest, Sample1) { EXPECT_EQ(solve(2, 11), true); }

TEST(SolverATest, Sample2) { EXPECT_EQ(solve(2, 13), false); }

TEST(SolverATest, Sample3) { EXPECT_EQ(solve(100, 600), true); }
