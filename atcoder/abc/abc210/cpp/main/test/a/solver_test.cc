#include "a/solver.h"

#include "gtest/gtest.h"

TEST(SolverATest, Sample1) { EXPECT_EQ(solve(5, 3, 20, 15), 90); }

TEST(SolverATest, Sample2) { EXPECT_EQ(solve(10, 10, 100, 1), 1000); }
