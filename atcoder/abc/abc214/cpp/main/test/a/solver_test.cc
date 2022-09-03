#include "a/solver.h"

#include "gtest/gtest.h"

TEST(SolverATest, Sample1) { EXPECT_EQ(solve(214), 8); }

TEST(SolverATest, Sample2) { EXPECT_EQ(solve(1), 4); }

TEST(SolverATest, Sample3) { EXPECT_EQ(solve(126), 6); }
