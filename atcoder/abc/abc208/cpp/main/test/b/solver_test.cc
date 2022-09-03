#include "b/solver.h"

#include "gtest/gtest.h"

TEST(SolverBTest, Sample1) { EXPECT_EQ(solve(9), 3); }
TEST(SolverBTest, Sample2) { EXPECT_EQ(solve(119), 10); }
TEST(SolverBTest, Sample3) { EXPECT_EQ(solve(10000000), 24); }
