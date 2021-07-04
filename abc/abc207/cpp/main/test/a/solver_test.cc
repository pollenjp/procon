#include "a/solver.h"

#include "gtest/gtest.h"

TEST(SolverATest, Sample1) { EXPECT_EQ(solve(3, 4, 5), 9); }

TEST(SolverATest, Sample2) { EXPECT_EQ(solve(6, 6, 6), 12); }

TEST(SolverATest, Sample3) { EXPECT_EQ(solve(99, 99, 98), 198); }
