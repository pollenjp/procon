#include "b/solver.h"

#include "gtest/gtest.h"

TEST(SolverATest, Sample1) { EXPECT_EQ(solve(6), 2); }

TEST(SolverATest, Sample2) { EXPECT_EQ(solve(1), 0); }

TEST(SolverATest, Sample3) { EXPECT_EQ(solve(1000000000000000000), 59); }
