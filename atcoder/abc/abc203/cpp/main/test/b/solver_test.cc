#include "b/solver.h"

#include "gtest/gtest.h"

TEST(SolverBTest, Sample1) { EXPECT_EQ(solve(1, 2), 203); }

TEST(SolverBTest, Sample2) { EXPECT_EQ(solve(3, 3), 1818); }
