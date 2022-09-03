#include "a/solver.h"

#include "gtest/gtest.h"

TEST(SolverATest, Sample1) { EXPECT_EQ(solve(180), "Yay!"); }

TEST(SolverATest, Sample2) { EXPECT_EQ(solve(200), ":("); }

TEST(SolverATest, Sample3) { EXPECT_EQ(solve(191), "so-so"); }
